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

    // const versions: Array<string> = await fetch("https://ddragon.leagueoflegends.com/api/versions.json")
    //     .then(resp => resp.json());
    // const champions: Champions = await fetch(`https://ddragon.leagueoflegends.com/cdn/${versions[0]}/data/en_US/champion.json`)
    //     .then(resp => resp.json());

    document
        .getElementById("summonerSelect")!
        .addEventListener("change", summonerSelectListener());
})()

function summonerSelectListener() {
    return function (this: HTMLSelectElement, ev: Event) {
        const sidebar = document.getElementById("sidebar");
        sidebar.replaceChildren();

        const summonerId = this.value;
        console.log(summonerId);

        document.getElementById("matches").replaceChildren();
        getSummonerGames(summonerId)
            .then(constructMatchHistory(0));

        const percentConfig = {
            style: 'percent',
            minimumFractionDigits: 0,
            maximumFractionDigits: 1
        };

        getSummonerStats(summonerId).then((stats) => {
            fetch(`https://api.påsan.com/summoner/${summonerId}`)
                .then((res) => res.json())
                .then((summoner: GamesPlayed) => {
                    stats
                        .sort((a, b) => (b.wins + b.losses) - (a.wins + a.losses))
                        .forEach((val) => {
                            const square_url = `https://cdn.communitydragon.org/latest/champion/${val.championId}/square`;
                            const games = val.wins + val.losses;
                            const pickrate_formatted = (games / summoner.games).toLocaleString(undefined, percentConfig);
                            const banrate_formatted = (val.bans / summoner.games).toLocaleString(undefined, percentConfig);
                            const winrate = Math.round((val.wins / games) * 100);
                            const winrate_formatted = (val.wins / games).toLocaleString(undefined, percentConfig);
                            const gamesText = games != 1 ? "games" : "game";
                            const winsText = val.wins != 1 ? "wins" : "win";
                            const lossesText = val.losses != 1 ? "losses" : "loss";
                            const bansText = val.bans != 1 ? "bans" : "ban";

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
                                    <div className="stats">
                                        <img className="stats-icon" width="48" src={square_url}></img>
                                        <div>{`${kills} / ${deaths} / ${assists} (${kda} KDA)`}</div>
                                    </div>
                                    <span>
                                        {`Pickrate: ${pickrate_formatted} (${games} ${gamesText})`}<br />
                                        {`Winrate: ${winrate_formatted} (${val.wins} ${winsText} / ${val.losses} ${lossesText})`}<br />
                                        {`Banrate: ${banrate_formatted} (${val.bans} ${bansText})`}
                                    </span>
                                </article>
                            );
                        });

                })
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
            var playerStats: LolMatchHistoryMatchHistoryParticipant;
            match.participants.forEach((participant) => {
                if (participant.participantId === participantId) {
                    playerStats = participant;
                }
            });
            const result = playerStats.stats.win ? "win" : "loss";
            const squareUrl = `https://cdn.communitydragon.org/latest/champion/${playerStats.championId}/square`;
            const kdaString = `${playerStats.stats.kills} / ${playerStats.stats.deaths} / ${playerStats.stats.assists}`;

            const highestDamage = Math.max(...match.participants.map((val) => val.stats.totalDamageDealtToChampions));

            const team1Container: HTMLDivElement = <div className="team1"></div>;
            const team2Container: HTMLDivElement = <div className="team2"></div>;
            match.participantIdentities.map((e, i) => [e, match.participants[i]]).forEach((e, i) => {
                const pi = e[0] as LolMatchHistoryMatchHistoryParticipantIdentities;
                const p = e[1] as LolMatchHistoryMatchHistoryParticipant;

                const squareUrl = `https://cdn.communitydragon.org/latest/champion/${p.championId}/square`;
                var summonerName = (
                    <a href="#" onclick={changeSelect(pi.player.summonerId)}>
                        {`${pi.player.summonerName}`}
                    </a>
                );
                if (pi.player.summonerId.toString() === summonerId) {
                    summonerName = <span><b>{pi.player.summonerName}</b></span>;
                }
                const summonerContainer = (
                    <div>
                        <img className="match-summoners-icon" src={squareUrl} width="32"></img>
                        {summonerName}
                        <span>{` (${p.stats.kills}/${p.stats.deaths}/${p.stats.assists})`}</span><br />
                        <meter className="damage-meter" value={p.stats.totalDamageDealtToChampions} min="0" max={highestDamage}></meter>
                        <span className="damage-text">{` ${p.stats.totalDamageDealtToChampions.toLocaleString()} damage`}</span>
                    </div>
                );

                if (i < 5) {
                    team1Container.appendChild(summonerContainer);
                } else {
                    team2Container.appendChild(summonerContainer);
                }
            });

            matchesDiv!.appendChild(
                <details className={result}>
                    <summary>
                        <img src={squareUrl} className="champion-icon" width="48"></img>
                        <span>{kdaString}</span>
                        <span className="match-date">{time.toLocaleDateString()}</span>
                    </summary>
                    <div className="summoners">
                        {team1Container}
                        {team2Container}
                    </div>
                </details>
            );
        });
        if (match_list.length != 0) {
            const fetchMoreMatchesButton = <button id="fetch-matches" type="button">Load more...</button>;
            fetchMoreMatchesButton.addEventListener("click", fetchMoreMatchesListener(offset + 20));
            matchesDiv?.appendChild(fetchMoreMatchesButton);
        }
    }
}

function changeSelect(summonerId: number | string) {
    return () => {
        const select = (document.getElementById("summonerSelect") as HTMLSelectElement)
        select.value = String(summonerId);
        var event = new Event('change');
        select.dispatchEvent(event);
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
    summonerId: number,
    wins: number,
    losses: number,
    kills: number,
    deaths: number,
    assists: number,
    bans: number,
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
    winrate: number,
    champs: number,
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