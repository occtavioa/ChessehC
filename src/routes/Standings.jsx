import { Table } from "react-bootstrap";
import { useLoaderData } from "react-router";

function Standings() {
    const standings = useLoaderData()
    
    return (<>
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
