import { useEffect, useState } from "react"
import { getPointValue } from "../utils"
import { invoke } from "@tauri-apps/api"
import { useParams } from "react-router"

function GameRow({ white, black, game }) {
    const { path } = useParams()
    const [state, setResult] = useState(game.state === "Ongoing" ? "Ongoing" : game.state.Finished)

    return (
        <tr>
            <PlayerData player={white} />
            <td>
                {
                    state !== "Ongoing" ?
                        <>{getPointValue(state.at(0))} - {getPointValue(state.at(1))}</> :
                        <form onSubmit={(e) => {
                            e.preventDefault()
                            let result = Object.fromEntries(new FormData(e.target))
                            let [whitePoint, blackPoint, ..._] = result.gameResult.split(',')
                            invoke("set_game_result", { gameId: game.id, whitePoint: whitePoint, blackPoint: blackPoint, path: atob(path) })
                                .then(() => {
                                    setResult([whitePoint, blackPoint])
                                })
                                .catch(e => {
                                    console.error(e);
                                })
                        }}>
                            <select name="gameResult" defaultValue={"D,D"}>
                                <option value={"W,L"}>{getPointValue("W")} - {getPointValue("L")}</option>
                                <option value={"D,D"}>{getPointValue("D")} - {getPointValue("D")}</option>
                                <option value={"L,W"}>{getPointValue("L")} - {getPointValue("W")}</option>
                            </select>
                            <button type="submit">Set</button>
                        </form>
                }
            </td>
            <PlayerData player={black} />
        </tr>
    )
}

function PlayerData({player}) {
    return (
        <>
            <td>{player.id}</td>
            <td>{player.rating}</td>
            <td>{player.title ?? <>-</>}</td>
            <td>{player.name}</td>
        </>
    )
}

export default GameRow
