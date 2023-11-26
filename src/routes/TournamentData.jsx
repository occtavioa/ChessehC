import { invoke } from "@tauri-apps/api";
import { Button, Table } from "react-bootstrap";
import { useLoaderData, useParams } from "react-router-dom";

function TournamentData() {
    const {path} = useParams()
    let tournament = useLoaderData()

    return (
        <>
            <Button onClick={() => {
                invoke("make_trf_file", {path: atob(path)})
                    .then((trfPath) => {
                        console.log("TRF exported at", trfPath);
                    })
                    .catch((e) => {
                        console.error(e);
                    })
            }}>Exportar archivo TRF</Button>
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
