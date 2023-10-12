function ByeRow({player, bye}) {
    return (
        <tr>
            <td>{player.id}</td>
            <td>{player.rating}</td>
            <td>{player.name}</td>
            <td>{getPointValue(bye.bye_point)}</td>
        </tr>
    )
}

export default ByeRow
