import { invoke } from "@tauri-apps/api"
import { useEffect, useRef, useState } from "react"
import { useParams } from "react-router"

function Pairings() {
    const {path, roundId} = useParams()
    const [games, setGames] = useState([])
    const [byes, setByes] = useState([])
    const [players, setPlayers] = useState([])

    useEffect(() => {
        invoke("get_pairings_by_round", {path: atob(path), roundId: parseInt(roundId)})
            .then((pairings) => {
                console.log(pairings);
                setGames(pairings.at(0))
                setByes(pairings.at(1))
            })
            .catch((error) => {console.error(error);})
    }, [path, roundId])

    useEffect(() => {
        console.log(games);
    }, [games])

    useEffect(() => {
        console.log(byes);
    }, [byes])
    
    return (
        <>
            <table>
                <caption>Partidas</caption>
                <thead>
                    <tr>
                        <th>Id</th>
                        <th>Rating</th>
                        <th>Nombre</th>
                        <th>Resultado</th>
                        <th>Id</th>
                        <th>Rating</th>
                        <th>Nombre</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        games.map(g => 
                            <tr key={g.id}>
                                <td>{g.white_id}</td>
                                <td>{g.black_id}</td>
                                <td>
                                    {
                                        g.state === "Ongoing" ?
                                            <>
                                                <form onSubmit={async (e) => {
                                                    e.preventDefault()
                                                    let result = Object.fromEntries(new FormData(e.target))
                                                    let [whitePoint, blackPoint, ..._] = result.gameResult.split(',')
                                                    invoke("set_game_result", {gameId: g.id, whitePoint: whitePoint, blackPoint: blackPoint, path: atob(path)})
                                                        .then((r) => {
                                                            
                                                        })
                                                        .catch(e => {
                                                            console.error(e);
                                                        })
                                                }}>
                                                    <select name="gameResult" defaultValue={"D,D"}>
                                                        <option value={"W,L"}>1 - 0</option>
                                                        <option value={"D,D"}>1/2 - 1/2</option>
                                                        <option value={"L,W"}>0 - 1</option>
                                                    </select>
                                                    <button type="submit">Set</button>
                                                </form>
                                            </> :
                                            <>
                                                {g.state.Finished.at(0)} - {g.state.Finished.at(1)}
                                            </>
                                    }
                                </td>
                            </tr>
                        )
                    }
                </tbody>
            </table>
            <table>
                <caption>Byes</caption>
                <thead>
                    <tr>
                        <th>Jugador</th>
                        <th>Punto</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        byes.map(b =>
                            <tr key={b.id}>
                                <td>{b.player_id}</td>
                                <td>{b.bye_point}</td>
                            </tr>
                        )
                    }
                </tbody>
            </table>
        </>
    )
}

export default Pairings
