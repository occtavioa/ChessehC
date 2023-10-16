import { useLoaderData } from "react-router"
import GameRow from "../components/GameRow"
import ByeRow from "../components/ByeRow"

function Pairings() {
    const {players, games, byes} = useLoaderData()

    return (
        <>
            <table>
                <caption>Partidas</caption>
                <GameHeader />
                <GameBody games={games} players={players} />
            </table>
            <table>
                <caption>Byes</caption>
                <ByeHeader />
                <ByeBody byes={byes} players={players} />
            </table>
        </>
    )
}

function ByeBody({byes, players}) {
    return (
        <tbody>
            {
                byes.map((bye, i) =>
                    <ByeRow
                        bye={bye}
                        player={players.find(player => player.id === bye.player_id)}
                        key={i}
                    />
                )
            }
        </tbody>
    )
}

function GameBody({games, players}) {
    return (
        <tbody>
            {
                games.map(game => 
                    <GameRow 
                        white={players.find(p => p.id === game.white_id)}
                        black={players.find(p => p.id === game.black_id)}
                        game={game}
                        key={game.id}
                    />
                )
            }
        </tbody>
    )
}

function GameHeader() {
    return (
        <thead>
            <tr>
                <GamePlayerHeader />
                <th>Resultado</th>
                <GamePlayerHeader />
            </tr>
        </thead>
    )
}

function GamePlayerHeader() {
    return (
        <>
            <th>Id</th>
            <th>Rating</th>
            <th>Título</th>
            <th>Nombre</th>
        </>
    )
}

function ByeHeader() {
    return (
        <thead>
            <tr>
                <th>Id</th>
                <th>Rating</th>
                <th>Título</th>
                <th>Nombre</th>
                <th>Punto</th>
            </tr>
        </thead>
    )
}

export default Pairings
