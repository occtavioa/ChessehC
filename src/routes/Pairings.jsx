import { invoke } from "@tauri-apps/api"
import { useEffect, useRef, useState } from "react"
import { useParams } from "react-router"

function Pairings() {
    const {path, round} = useParams()
    const [pairings, setPairings] = useState([])

    useEffect(() => {
        invoke("get_pairings_by_round", {path: atob(path), round: parseInt(round)})
            .then((pairings) => {
                setPairings(pairings)
            })
            .catch((error) => {console.error(error);})
    }, [round])

    return (
        <>
            Ronda {round}
            <table>
                <thead>
                    <tr>
                        <th>
                            Blancas
                        </th>
                        <th>
                            Negras
                        </th>
                        <th>
                            Resultado
                        </th>
                    </tr>
                </thead>
                <tbody>
                    {
                        pairings.map((p, i) => 
                            <tr key={i}>
                                {
                                    p.kind.Game ?
                                        <>
                                            <td>{p.kind.Game.white_player.name}</td>
                                            <td>{p.kind.Game.black_player.name}</td>
                                            <td>
                                                {p.kind.Game.white_result} - {p.kind.Game.black_result}
                                            </td>
                                        </> :
                                        <>
                                            <td>{p.kind.Bye.player.name}</td>
                                            <td><b>Bye</b></td>
                                            <td>{p.kind.Bye.bye_point}</td>
                                        </>
                                }
                            </tr>
                        )
                    }
                </tbody>
            </table>
        </>
    )
}

export default Pairings
