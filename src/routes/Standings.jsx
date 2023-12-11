import { useEffect, useState } from "react";
import { Alert, AlertHeading, ListGroup, Table } from "react-bootstrap";
import { useLoaderData } from "react-router";

function Standings() {
    const {standings, status} = useLoaderData()
    const [podium, setPodium] = useState([])

    useEffect(() => {
        if(status === "Finished")
            setPodium(standings.slice(0, 3))
    }, [status])

    return (<>
        {
            status === "Finished"
            && (
                <Alert variant="success">
                    <Alert.Heading>
                        Clasificación final
                    </Alert.Heading>
                    <ListGroup>
                        <ListGroup.Item>
                            <i className="bi bi-1-circle-fill"></i>
                            <p>1er puesto: {standings[0].name}</p>
                        </ListGroup.Item>
                        <ListGroup.Item>
                            <i className="bi bi-2-circle-fill"></i>
                            <p>2do puesto: {standings[1].name}</p>
                        </ListGroup.Item>
                        <ListGroup.Item>
                            <i className="bi bi-3-circle-fill"></i>
                            <p>3er puesto: {standings[2].name}</p>
                        </ListGroup.Item>
                    </ListGroup>
                </Alert>
            )
        }
        <Table>
            <caption>Clasificación</caption>
            <thead>
                <tr>
                    <th>Id</th>
                    <th>Rating</th>
                    <th>Título</th>
                    <th>Nombre</th>
                    <th>Puntos</th>
                </tr>
            </thead>
            <tbody>
                {
                    standings.map((p, i) =>
                        <tr key={i}>
                            <td>{p.id}</td>
                            <td>{p.rating}</td>
                            <td>{p.title ?? <>-</>}</td>
                            <td>{p.name}</td>
                            <td>{p.points}</td>
                        </tr>
                    )
                }
            </tbody>
        </Table>
    </>)
}

export default Standings
