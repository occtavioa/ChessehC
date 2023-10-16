import { getPointValue } from "../utils"

function ByeRow({player, bye}) {
    return (
        <tr>
            <td>{player.id}</td>
            <td>{player.rating}</td>
            <td>{player.title ?? <>-</>}</td>
            <td>{player.name}</td>
            <td>{getPointValue(bye.bye_point)}</td>
        </tr>
    )
}

export default ByeRow
