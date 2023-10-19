(async () => {
    fetch("https://api.påsan.com/summoner_names")
        .then(resp => resp.json())
        .then((summonerList: Array<GamesPlayed>) => {
            const sel = document.getElementById("summonerSelect");
            summonerList
                .sort((a, b) => a.summonerName.localeCompare(b.summonerName))
                .forEach((summoner) => {
                    sel!.append(
                        <option value={summoner.summonerId.toString()}>
                            {summoner.summonerName}
                        </option>
                    );
                });
        })

    const versions: Array<string> = await fetch("https://ddragon.leagueoflegends.com/api/versions.json")
        .then(resp => resp.json());
    const champions: Champions = await fetch(`https://ddragon.leagueoflegends.com/cdn/${versions[0]}/data/en_US/champion.json`)
        .then(resp => resp.json());

    document
        .getElementById("summonerSelect")!
        .addEventListener("change", summonerSelectListener(champions));

    console.log(champions);
})()

function summonerSelectListener(champions: Champions) {
    return function (this: HTMLSelectElement, ev: Event) {
        const sidebar = document.getElementById("sidebar");
        sidebar.replaceChildren();

        const summonerId = this.value;
        console.log(summonerId);

        document.getElementById("matches").replaceChildren();
        getSummonerGames(summonerId)
            .then(constructMatchHistory(0));

        getSummonerStats(summonerId).then((stats) => {
            stats
                .sort((a, b) => (b.wins + b.losses) - (a.wins + a.losses))
                .forEach((val) => {
                    const square_url = `https://cdn.communitydragon.org/latest/champion/${val.championId}/square`;
                    const games = val.wins + val.losses;
                    const winrate = Math.round((val.wins / games) * 100);
                    const winrate_formatted = (val.wins / games)
                        .toLocaleString(undefined, {
                            style: 'percent',
                            minimumFractionDigits: 0,
                            maximumFractionDigits: 1
                        });
                    const games_text = games != 1 ? "games" : "game";

                    const format_opts = {
                        minimumFractionDigits: 0,
                        maximumFractionDigits: 1
                    };
                    const kills = (val.kills / games)
                        .toLocaleString(undefined, format_opts);
                    const deaths = (val.deaths / games)
                        .toLocaleString(undefined, format_opts);
                    const assists = (val.assists / games)
                        .toLocaleString(undefined, format_opts);

                    var kda: string;
                    if (val.deaths === 0) {
                        kda = "Perfect";
                    } else {
                        kda = ((val.kills + val.assists) / val.deaths)
                            .toLocaleString(undefined, format_opts);
                    }

                    sidebar.appendChild(
                        <article className="champion-stat" style={`--winrate: ${winrate}`}>
                            <img className="stats-icon" width="48" src={square_url}></img>
                            <div className="stats">{`${winrate_formatted} (${games_text})`}</div>
                            <div>{`${kills} / ${deaths} / ${assists} (${kda} KDA)`}</div>
                        </article>
                    );
                });
        })

    }
}

function fetchMoreMatchesListener(offset: number) {
    return function (ev) {
        document.getElementById("fetch-matches")?.remove();
        const summonerId = (document.getElementById("summonerSelect") as HTMLSelectElement).value;
        getSummonerGames(summonerId, 20, offset).then(constructMatchHistory(offset));
    }
}

function constructMatchHistory(offset: number) {
    return function (match_list: Array<LolMatchHistoryMatchHistoryGame>) {
        const matchesDiv = document.getElementById("matches");

        const summonerId = (document.getElementById("summonerSelect") as HTMLSelectElement).value;
        match_list.forEach((match) => {
            const time = new Date(match.gameCreation);

            var participantId: number;
            match.participantIdentities.forEach((participantIdentity) => {
                if (participantIdentity.player.summonerId.toString() === summonerId) {
                    participantId = participantIdentity.participantId;
                }
            });
            var player_stats: LolMatchHistoryMatchHistoryParticipant;
            match.participants.forEach((participant) => {
                if (participant.participantId === participantId) {
                    player_stats = participant;
                }
            });
            // const details = document.createElement("details");
            const result = player_stats.stats.win ? "win" : "loss";
            const square_url = `https://cdn.communitydragon.org/latest/champion/${player_stats.championId}/square`;
            const kda_string = `${player_stats.stats.kills} / ${player_stats.stats.deaths} / ${player_stats.stats.assists}`;

            matchesDiv!.appendChild(
                <details className={result}>
                    <summary>
                        <img src={square_url} className="champion-icon" width="48"></img>
                        <span>{kda_string}</span>
                        <span className="match-date">{time.toLocaleDateString()}</span>
                    </summary>
                    <p>test</p>
                </details>
            );
        });
        if (match_list.length != 0) {
            const fetchMoreMatchesButton = <button id="fetch-matches">Load more...</button>;
            fetchMoreMatchesButton.addEventListener("click", fetchMoreMatchesListener(offset + 20));
            matchesDiv?.appendChild(fetchMoreMatchesButton);
        }
    }
}

async function getSummonerGames(
    summonerId: number | string,
    amount: number = 20,
    offset: number = 0
): Promise<Array<LolMatchHistoryMatchHistoryGame>> {
    const url = `https://api.påsan.com/summoner_matches/${summonerId}?amount=${amount}&offset=${offset}`;
    return await fetch(url).then(resp => resp.json());
}

async function getSummonerStats(
    summonerId: number | string
): Promise<Array<ChampionStats>> {
    const url = `https://api.påsan.com/summoner_stats/${summonerId}`;
    return await fetch(url).then(resp => resp.json());
}

interface ChampionStats {
    championId: number,
    wins: number,
    losses: number,
    kills: number,
    deaths: number,
    assists: number,
}


interface LolMatchHistoryMatchHistoryGame {
    id: number,
    gameId: number,
    platformId: string,
    gameCreation: number,
    gameCreationDate: string,
    gameDuration: number,
    queueId: number,
    mapId: number,
    seasonId: number,
    gameVersion: string,
    gameMode: string,
    gameType: string,
    teams: Array<LolMatchHistoryMatchHistoryTeam>,
    participants: Array<LolMatchHistoryMatchHistoryParticipant>,
    participantIdentities: Array<LolMatchHistoryMatchHistoryParticipantIdentities>,
}

interface LolMatchHistoryMatchHistoryParticipantIdentities {
    participantId: number,
    player: LolMatchHistoryMatchHistoryParticipantIdentityPlayer,
}

interface LolMatchHistoryMatchHistoryParticipantIdentityPlayer {
    platformId: string,
    accountId: number,
    summonerId: number,
    summonerName: string,
    currentPlatformId: string,
    currentAccountId: number,
    matchHistoryUri: string,
    profileIcon: number,
}

interface LolMatchHistoryMatchHistoryTeam {
    teamId: number,
    win: string,
    firstBlood: boolean,
    firstTower: boolean,
    firstInhibitor: boolean,
    firstBaron: boolean,
    firstDargon: boolean,
    towerKills: number,
    inhibitorKills: number,
    baronKills: number,
    dragonKills: number,
    vilemawKills: number,
    riftHeraldKills: number,
    dominionVictoryScore: number,
    bans: Array<LolMatchHistoryMatchHistoryTeamBan>,
}

interface LolMatchHistoryMatchHistoryTeamBan {
    championId: number,
    pickTurn: number,
}

interface ChampionWinrate {
    championId: number;
    wins: number;
    losses: number;
    kills: number;
    deaths: number;
    assists: number;
}

interface GamesPlayed {
    summonerId: number;
    summonerName: string;
    games: number;
}

interface LolMatchHistoryMatchHistoryParticipantStatistics {
    participantId: number;
    win: boolean;
    item0: number;
    item1: number;
    item2: number;
    item3: number;
    item4: number;
    item5: number;
    item6: number;
    kills: number;
    deaths: number;
    assists: number;
    largestKillingSpree: number;
    largestMultiKill: number;
    killingSprees: number;
    longestTimeSpentLiving: number;
    doubleKills: number;
    tripleKills: number;
    quadraKills: number;
    pentaKills: number;
    unrealKills: number;
    totalDamageDealt: number;
    magicDamageDealt: number;
    physicalDamageDealt: number;
    trueDamageDealt: number;
    largestCriticalStrike: number;
    totalDamageDealtToChampions: number;
    magicDamageDealtToChampions: number;
    physicalDamageDealtToChampions: number;
    trueDamageDealtToChampions: number;
    totalHeal: number;
    totalUnitsHealed: number;
    totalDamageTaken: number;
    magicalDamageTaken: number;
    physicalDamageTaken: number;
    trueDamageTaken: number;
    goldEarned: number;
    goldSpent: number;
    turretKills: number;
    inhibitorKills: number;
    totalMinionsKilled: number;
    neutralMinionsKilled: number;
    neutralMinionsKilledTeamJungle: number;
    neutralMinionsKilledEnemyJungle: number;
    totalTimeCrowdControlDealt: number;
    champLevel: number;
    visionWardsBoughtInGame: number;
    sightWardsBoughtInGame: number;
    wardsPlaced: number;
    wardsKilled: number;
    firstBloodKill: boolean;
    firstBloodAssist: boolean;
    firstTowerKill: boolean;
    firstTowerAssist: boolean;
    firstInhibitorKill: boolean;
    firstInhibitorAssist: boolean;
    gameEndedInEarlySurrender: boolean;
    gameEndedInSurrender: boolean;
    causedEarlySurrender: boolean;
    earlySurrenderAccomplice: boolean;
    teamEarlySurrendered: boolean;
    combatPlayerScore: number;
    objectivePlayerScore: number;
    totalPlayerScore: number;
    totalScoreRank: number;
    damageSelfMitigated: number;
    damageDealtToObjectives: number;
    damageDealtToTurrets: number;
    visionScore: number;
    timeCCingOthers: number;
    playerScore0: number;
    playerScore1: number;
    playerScore2: number;
    playerScore3: number;
    playerScore4: number;
    playerScore5: number;
    playerScore6: number;
    playerScore7: number;
    playerScore8: number;
    playerScore9: number;
    perkPrimaryStyle: number;
    perkSubStyle: number;
    perk0: number;
    perk0Var1: number;
    perk0Var2: number;
    perk0Var3: number;
    perk1: number;
    perk1Var1: number;
    perk1Var2: number;
    perk1Var3: number;
    perk2: number;
    perk2Var1: number;
    perk2Var2: number;
    perk2Var3: number;
    perk3: number;
    perk3Var1: number;
    perk3Var2: number;
    perk3Var3: number;
    perk4: number;
    perk4Var1: number;
    perk4Var2: number;
    perk4Var3: number;
    perk5: number;
    perk5Var1: number;
    perk5Var2: number;
    perk5Var3: number;
}

interface LolMatchHistoryMatchHistoryTimeline {
    participantId: number;
    role: string;
    lane: string;
    creepsPerMinDeltas: Object,
    xpPerMinDeltas: Object,
    goldPerMinDeltas: Object,
    csDiffPerMinDeltas: Object,
    xpDiffPerMinDeltas: Object,
    damageTakenPerMinDeltas: Object,
    damageTakenDiffPerMinDeltas: Object
}

interface LolMatchHistoryMatchHistoryParticipant {
    participantId: number;
    teamId: number;
    championId: number;
    spell1Id: number;
    spell2Id: number;
    highestAchievedSeasonTier: string;
    stats: LolMatchHistoryMatchHistoryParticipantStatistics;
    timeline: LolMatchHistoryMatchHistoryTimeline;
}

interface Champions {
    type: string;
    format: string;
    version: string;
    data: {
        [key: string]: Champion
    };
}

interface Champion {
    version: string;
    id: string;
    key: string;
    name: string;
    title: string;
    blurb: string;
    info: {
        attack: number;
        defense: number;
        magic: number;
        difficulty: number;
    };
    image: {
        full: string;
        sprite: string;
        group: string;
        x: string;
        y: string;
        w: string;
        h: string;
    };
    tags: Array<string>;
    partype: string;
    stats: {
        hp: number;
        hpperlevel: number;
        mp: number;
        mpperlevel: number;
        movespeed: number;
        armor: number;
        armorperlevel: number;
        spellblock: number;
        spellblockperlevel: number;
        attackrange: number;
        hpregen: number;
        hpregenperlevel: number;
        mpregen: number;
        mpregenperlevel: number;
        crit: number;
        critperlevel: number;
        attackdamage: number;
        attackdamageperlevel: number;
        attackspeedperlevel: number;
        attackspeed: number;
    }
}

function nonNull(val, fallback) { return Boolean(val) ? val : fallback };

function DOMparseChildren(children) {
    return children.map(child => {
        if (typeof child === 'string') {
            return document.createTextNode(child);
        }
        return child;
    })
}

function DOMparseNode(element, properties, children) {
    const el = document.createElement(element);
    Object.keys(nonNull(properties, {})).forEach(key => {
        el[key] = properties[key];
    })
    DOMparseChildren(children).forEach(child => {
        el.appendChild(child);
    });
    return el;
}

function DOMcreateElement(element, properties, ...children) {
    if (typeof element === 'function') {
        return element({
            ...nonNull(properties, {}),
            children
        });
    }
    return DOMparseNode(element, properties, children);
}