(async () => {
    fetch("https://api.påsan.com/summoner_names")
        .then(resp => resp.json())
        .then((summonerList: Array<GamesPlayed>) => {
            const sel = document.getElementById("summonerSelect");
            summonerList
                .sort((a, b) => a.summonerName.localeCompare(b.summonerName))
                .forEach((summoner) => {
                    const opt = document.createElement("option");
                    opt.value = String(summoner.summonerId);
                    opt.text = summoner.summonerName;
                    sel?.append(opt);
                });
        })

    const versions: Array<string> = await fetch("https://ddragon.leagueoflegends.com/api/versions.json")
        .then(resp => resp.json());
    const champions: Champions = await fetch(`https://ddragon.leagueoflegends.com/cdn/${versions[0]}/data/en_US/champion.json`)
        .then(resp => resp.json());

    document
        .getElementById("summonerSelect")
        ?.addEventListener("change", summonerSelectListener(champions));


    console.log(champions);
})()

function summonerSelectListener(champions: Champions) {
    return function (this: HTMLSelectElement, ev: Event) {
        const main = document.getElementById("main");
        main.replaceChildren();

        const summonerId = this.value;
        console.log(summonerId);
        get_summoner_games(summonerId)
            .then((match_list: Array<LolMatchHistoryMatchHistoryParticipant>) => {
                match_list.reverse();
                const matches_div = document.createElement("div");
                main?.appendChild(matches_div);
                matches_div.className = "matches";

                const champs = new Map<number, ChampionWinrate>;
                match_list.forEach((match) => {
                    if (!champs.has(match.championId)) {
                        champs.set(match.championId, {
                            championId: match.championId,
                            wins: 0,
                            losses: 0,
                            kills: 0,
                            deaths: 0,
                            assists: 0,
                        });
                    }

                    const champ = champs.get(match.championId);
                    if (champ != undefined) {
                        champ.kills += match.stats.kills;
                        champ.deaths += match.stats.deaths;
                        champ.assists += match.stats.assists;
                        if (match.stats.win) {
                            champ.wins += 1;
                        } else {
                            champ.losses += 1;
                        }
                    }

                    const details = document.createElement("details");
                    details.className = match.stats.win ? "win" : "loss";

                    const summary = document.createElement("summary");
                    const champ_icon = document.createElement("img");
                    const game_result = document.createElement("span");
                    game_result.textContent = `${match.stats.kills} / ${match.stats.deaths} / ${match.stats.assists}`;
                    champ_icon.src = `https://cdn.communitydragon.org/latest/champion/${match.championId}/square`;
                    champ_icon.className = "champion-icon";
                    champ_icon.width = 48;
                    summary.appendChild(champ_icon);
                    summary.appendChild(game_result);

                    const p = document.createElement("p");
                    p.innerHTML = "test";

                    details.appendChild(summary);
                    details.appendChild(p);
                    matches_div?.appendChild(details);
                });

                const sidebar = document.createElement("div");
                main?.prepend(sidebar);
                sidebar.className = "sidebar";
                Array.from(champs.values())
                    .sort((a, b) => (b.wins + b.losses) - (a.wins + a.losses))
                    .forEach((val) => {
                        const article = document.createElement("article");
                        article.className = "champion-stat";

                        const champ_icon = document.createElement("img");
                        champ_icon.src = `https://cdn.communitydragon.org/latest/champion/${val.championId}/square`;
                        champ_icon.className = "stats-icon";
                        champ_icon.width = 48;
                        article.appendChild(champ_icon);

                        const stats1 = document.createElement("div");
                        stats1.className = "stats";
                        const games = val.wins + val.losses;
                        article.setAttribute("style", `--winrate: ${Math.round((val.wins / games) * 100)}`);
                        stats1.textContent = (val.wins / games)
                            .toLocaleString(undefined, {
                                style: 'percent',
                                minimumFractionDigits: 0,
                                maximumFractionDigits: 1
                            });

                        stats1.textContent += ` (${games} ${games != 1 ? "games" : "game"})`;

                        const stats2 = document.createElement("div");
                        const kills = (val.kills / games)
                            .toLocaleString(undefined, {
                                minimumFractionDigits: 0,
                                maximumFractionDigits: 1
                            });
                        const deaths = (val.deaths / games)
                            .toLocaleString(undefined, {
                                minimumFractionDigits: 0,
                                maximumFractionDigits: 1
                            });
                        const assists = (val.assists / games)
                            .toLocaleString(undefined, {
                                minimumFractionDigits: 0,
                                maximumFractionDigits: 1
                            });
                        var kda;
                        if (val.deaths === 0) {
                            kda = "Perfect";
                        } else {
                            kda = ((val.kills + val.assists) / val.deaths)
                                .toLocaleString(undefined, {
                                    minimumFractionDigits: 0,
                                    maximumFractionDigits: 2
                                });
                        }

                        stats2.textContent += `${kills} / ${deaths} / ${assists} (${kda} KDA)`;

                        article.appendChild(stats1);
                        article.appendChild(stats2);
                        sidebar.appendChild(article);
                    });
            })
    }
}

async function get_summoner_games(
    summonerId: number | string,
    amount: number = 1000,
    offset: number = 0
): Promise<Array<LolMatchHistoryMatchHistoryParticipant>> {
    const url = `https://api.påsan.com/summoner/${summonerId}?amount=${amount}&offset=${offset}`;
    return await fetch(url).then(resp => resp.json());
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