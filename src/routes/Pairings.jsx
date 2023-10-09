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
                setGames(pairings[0])
                setByes(pairings[1])
            })
            .catch((error) => {console.error(error);})
    }, [path, roundId])

    return (
        <>
            <table>
                <caption>Partidas</caption>
                <thead>
                    <tr>
                        <th>Blancas</th>
                        <th>Negras</th>
                        <th>Resultado</th>
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
                                                <select defaultValue={"D,D"}>
                                                    <option value={"W,L"}>1 - 0</option>
                                                    <option value={"D,D"}>1/2 - 1/2</option>
                                                    <option value={"L,W"}>0 - 1</option>
                                                </select>
                                                <button>Set</button>
                                            </> :
                                            <>
                                                {
                                                    g.Finished[0] - g.Finished[1]
                                                }
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
