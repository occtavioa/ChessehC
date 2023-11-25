import { Table } from "react-bootstrap";
import { useLoaderData } from "react-router-dom";

function TournamentData() {
    let tournament = useLoaderData()

    return (
        <>
            <Table>
                <thead>
                    <tr>
                        <th>Nombre</th>
                        <th>Número de rondas</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>{tournament.name}</td>
                        <td>{tournament.number_rounds}</td>
                    </tr>
                </tbody>
            </Table>
        </>
    )
}

export default TournamentData;
