(async () => {
    const percentConfig = {
        style: 'percent',
        minimumFractionDigits: 0,
        maximumFractionDigits: 1
    };

    const formatOpts = {
        minimumFractionDigits: 0,
        maximumFractionDigits: 1
    }
    createTable(
        <thead>
            <tr>
                <th>Summoner</th>
                <th>Games</th>
            </tr>
        </thead>,
        "games",
        "/summoner_names?order_by=games&cutoff=10&order=descending",
        (summoner: Summoner) =>
            <tr>
                <td>{summoner.summonerName}</td>
                <td>{`${summoner.games}`}</td>
            </tr>
    );
    createTable(
        <thead>
            <tr>
                <th>Summoner</th>
                <th>Winrate</th>
                <th>Games</th>
            </tr>
        </thead>,
        "winrate",
        "/summoner_names?order_by=winrate&cutoff=10&order=descending",
        (summoner: Summoner) =>
            <tr>
                <td>{summoner.summonerName}</td>
                <td>{summoner.winrate.toLocaleString(undefined, percentConfig)}</td>
                <td>{`${summoner.games}`}</td>
            </tr>
    );
    createTable(
        <thead>
            <tr>
                <th>Summoner</th>
                <th>KDA</th>
                <th>Games</th>
            </tr>
        </thead>,
        "highest-kda",
        "/kdas?cutoff=10&order=descending",
        (summoner: Kda) =>
            <tr>
                <td>{summoner.summonerName}</td>
                <td>{summoner.kda.toLocaleString(undefined, formatOpts)}</td>
                <td>{`${summoner.games}`}</td>
            </tr>
    );
    createTable(
        <thead>
            <tr>
                <th>Summoner</th>
                <th>KDA</th>
                <th>Games</th>
            </tr>
        </thead>,
        "lowest-kda",
        "/kdas?cutoff=10&order=ascending",
        (summoner: Kda) =>
            <tr>
                <td>{summoner.summonerName}</td>
                <td>{summoner.kda.toLocaleString(undefined, formatOpts)}</td>
                <td>{`${summoner.games}`}</td>
            </tr>
    );
    createTable(
        <thead>
            <tr>
                <th>Summoner</th>
                <th>CS</th>
                <th>Games</th>
            </tr>
        </thead>,
        "cs",
        "/stat_per_min/minions_killed?cutoff=10&order=descending",
        (summoner: Stat) =>
            <tr>
                <td>{summoner.summonerName}</td>
                <td>{summoner.stat.toLocaleString(undefined, formatOpts)}</td>
                <td>{`${summoner.games}`}</td>
            </tr>
    );
    createTable(
        <thead>
            <tr>
                <th>Summoner</th>
                <th>Damage dealt</th>
                <th>Games</th>
            </tr>
        </thead>,
        "damage-dealt",
        "/stat_per_min/damage_dealt?cutoff=10&order=descending",
        (summoner: Stat) =>
            <tr>
                <td>{summoner.summonerName}</td>
                <td>{summoner.stat.toLocaleString(undefined, formatOpts)}</td>
                <td>{`${summoner.games}`}</td>
            </tr>
    );
    createTable(
        <thead>
            <tr>
                <th>Summoner</th>
                <th>Damage taken</th>
                <th>Games</th>
            </tr>
        </thead>,
        "damage-taken",
        "/stat_per_min/damage_taken?cutoff=10&order=descending",
        (summoner: Stat) =>
            <tr>
                <td>{summoner.summonerName}</td>
                <td>{summoner.stat.toLocaleString(undefined, formatOpts)}</td>
                <td>{`${summoner.games}`}</td>
            </tr>
    );
    createTable(
        <thead>
            <tr>
                <th>Summoner</th>
                <th>Damage to turrets</th>
                <th>Games</th>
            </tr>
        </thead>,
        "damage-to-turrets",
        "/stat_per_min/damage_to_turrets?cutoff=10&order=descending",
        (summoner: Stat) =>
            <tr>
                <td>{summoner.summonerName}</td>
                <td>{summoner.stat.toLocaleString(undefined, formatOpts)}</td>
                <td>{`${summoner.games}`}</td>
            </tr>
    );
    createTable(
        <thead>
            <tr>
                <th>Summoner</th>
                <th>Damage to objectives</th>
                <th>Games</th>
            </tr>
        </thead>,
        "damage-to-objectives",
        "/stat_per_min/damage_to_objectives?cutoff=10&order=descending",
        (summoner: Stat) =>
            <tr>
                <td>{summoner.summonerName}</td>
                <td>{summoner.stat.toLocaleString(undefined, formatOpts)}</td>
                <td>{`${summoner.games}`}</td>
            </tr>
    );
    createTable(
        <thead>
            <tr>
                <th>Summoner</th>
                <th>Champions played</th>
                <th>Games</th>
            </tr>
        </thead>,
        "champions-played",
        "/summoner_names?order_by=champions_played&cutoff=10&order=descending",
        (summoner: Summoner) => {
            return <tr>
                <td>{summoner.summonerName}</td>
                <td>{`${summoner.champs}`}</td>
                <td>{`${summoner.games}`}</td>
            </tr>
        }
    );
    createTable(
        <thead>
            <tr>
                <th>Summoner</th>
                <th>Champion</th>
                <th>Wins</th>
            </tr>
        </thead>,
        "lossless",
        "/lossless_champions?cutoff=3",
        (summoner: SummonerChampion) => {
            return <tr>
                <td>{summoner.summonerName}</td>
                <td>
                    <img width="48" src={`https://cdn.communitydragon.org/latest/champion/${summoner.championId}/square`} />
                </td>
                <td>{`${summoner.wins}`}</td>
            </tr>
        }
    );
    createTable(
        <thead>
            <tr>
                <th>Summoner</th>
                <th>Winrate</th>
                <th>Games</th>
            </tr>
        </thead>,
        "lowest-winrate",
        "/summoner_names?order_by=winrate&cutoff=10&order=ascending",
        (summoner: Summoner) =>
            <tr>
                <td>{summoner.summonerName}</td>
                <td>{summoner.winrate.toLocaleString(undefined, percentConfig)}</td>
                <td>{`${summoner.games}`}</td>
            </tr>
    );
})()

async function createTable<T>(
    tableHead: Node,
    articleId: string,
    url: string,
    loopCallback: ((summoner: T) => Node)
): Promise<void> {
    fetch(`https://api.påsan.com${url}`)
        .then((resp) => resp.json())
        .then((summoners: Array<T>) => {
            const tableBody = <tbody></tbody>;
            const tableRows = summoners
                .slice(0, 10)
                .map(loopCallback);
            tableRows.forEach((summoner) => {
                tableBody.appendChild(summoner);
            });
            document.getElementById(articleId)?.appendChild(
                <table>
                    {tableHead}
                    {tableBody}
                </table>
            );
        })
}

interface Summoner {
    summonerId: number,
    summonerName: string,
    games: number,
    winrate: number,
    champs: number,
}

interface Stat {
    summonerId: number,
    summonerName: string,
    games: number,
    stat: number,
}

interface Kda {
    summonerId: number,
    summonerName: string,
    games: number,
    kills: number,
    deaths: number,
    assists: number,
    kda: number,
}

interface SummonerChampion {
    championId: number,
    summonerId: number,
    summonerName: string,
    wins: number,
    losses: number,
}