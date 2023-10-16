import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { useLoaderData, useParams } from "react-router";

function Standings() {
    const standings = useLoaderData()
    
    return (
        <>
            <table>
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
                                <td>{p.title}</td>
                                <td>{p.name}</td>
                                <td>{p.points}</td>
                            </tr>
                        )
                    }
                </tbody>
            </table>
        </>
    )
}

export default Standings
