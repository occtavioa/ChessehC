import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { Alert, Button, Table } from "react-bootstrap";
import { useLoaderData, useParams } from "react-router-dom";

function TournamentData() {
    const {path} = useParams()
    let tournament = useLoaderData()
    const [exportTrfResult, setExportTrfResult] = useState()

    useEffect(() => {
        console.log(exportTrfResult);
    }, [exportTrfResult])
    
    return (
        <>
            {
                typeof exportTrfResult !== "undefined"
                && (
                    <Alert variant={exportTrfResult.type === "success" ? "success" : "danger"}>
                        {exportTrfResult.message}
                    </Alert>
                )
            }
            <Button onClick={async () => {
                try {
                    let trf_path = await invoke("make_trf_file", {path: atob(path)})
                    setExportTrfResult({type: "success", message: `TRF exported at ${trf_path}`})
                } catch (error) {
                    setExportTrfResult({type: "error", message: "No deben haber rondas en juego"})
                }
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
