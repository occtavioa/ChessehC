import { useEffect, useState } from "react"
import GameRowPlayerData from "./GameRowPlayerData"

function GameRow({white, black, game}) {
    const [result, setResult] = useState()

    useEffect(() => {
        if(game.state !== "Ongoing") {
            setResult(game.state.Finished)
        }
    }, [game])

    return (
        <tr>
            <GameRowPlayerData player={white}></GameRowPlayerData>
            <td>
                {
                    result ?
                    <>{getPointValue(result.at(0))} - {getPointValue(result.at(1))}</> :
                    <form onSubmit={(e) => {
                        e.preventDefault()
                        let result = Object.fromEntries(new FormData(e.target))
                        let [whitePoint, blackPoint, ..._] = result.gameResult.split(',')
                        invoke("set_game_result", {gameId: game.id, whitePoint: whitePoint, blackPoint: blackPoint, path: atob(path)})
                        .then((pGame) => {
                            game = pGame
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
            <GameRowPlayerData player={black}></GameRowPlayerData>
        </tr>
    )
}

export default GameRow
