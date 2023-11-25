import { useEffect, useRef, useState } from "react"
import { Button, Form, Modal, Table } from "react-bootstrap";
import { useHref } from "react-router"
import { useFetcher } from "react-router-dom";

function Players() {
    const fetcher = useFetcher()
    const href = useHref()
    const [players, setPlayers] = useState([])
    const [showPlayerModal, setShowPlayerModal] = useState(false)

    useEffect(() => {
        if(fetcher.state === "idle" && !fetcher.data) {
            fetcher.load(href)
        }
    }, [fetcher.state])

    useEffect(() => {
        if(fetcher.data) {
            setPlayers(fetcher.data)
        }
    }, [fetcher.data])

    return (<>
        <Button onClick={() => {
            setShowPlayerModal(true)
        }}>Agregar jugador</Button>

        <Table>
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
                    players.map((p, i) => 
                        <tr key={i}>
                            <td>{p.id}</td>
                            <td>{p.rating}</td>
                            <td>{p.title}</td>
                            <td>{p.name}</td>
                            <td>{p.points}</td>
                        </tr>
                    )
                }
            </tbody>
        </Table>

        <Modal show={showPlayerModal} onHide={() => {setShowPlayerModal(false)}}>
            <Modal.Header closeButton>
                <Modal.Title>Agregar jugador</Modal.Title>
            </Modal.Header>
            <Modal.Body>
                <Form as={fetcher.Form} method="post" onSubmit={() => {
                    setShowPlayerModal(false)
                }}>
                    <Form.Group>
                        <Form.Label htmlFor="name">Nombre</Form.Label>
                        <Form.Control type="text" name="name" id="name" required/>
                    </Form.Group>

                    <Form.Group>
                        <Form.Label htmlFor="title">Título</Form.Label> 
                        <Form.Select name="title" id="title">
                            <option value="">Ninguno</option>
                            <option>WCM</option>
                            <option>WFM</option>
                            <option>CM</option>
                            <option>WIM</option>
                            <option>FM</option>
                            <option>WGM</option>
                            <option>IM</option>
                            <option>GM</option>
                        </Form.Select>
                    </Form.Group>

                    <Form.Group>
                        <Form.Label htmlFor="rating">Rating</Form.Label>
                        <Form.Control type="number" name="rating" id="rating" min={0} max={9999} required/>
                    </Form.Group>

                    <Button type="submit">Agregar</Button>
                </Form>
            </Modal.Body>
        </Modal>
    </>)
}

export default Players
