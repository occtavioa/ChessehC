import { invoke } from "@tauri-apps/api"
import { useEffect, useRef, useState } from "react"
import { useLoaderData, useParams } from "react-router"
import GameRow from "../components/GameRow"
import ByeRow from "../components/ByeRow"

function Pairings() {
    const {players, games, byes} = useLoaderData()

    return (
        <>
            <table>
                <caption>Partidas</caption>
                <thead>
                    <tr>
                        <th>Id</th>
                        <th>Rating</th>
                        <th>Nombre</th>
                        <th>Resultado</th>
                        <th>Id</th>
                        <th>Rating</th>
                        <th>Nombre</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        games.map(game => 
                            <GameRow 
                                white={players.find(p => p.id === game.white_id)}
                                black={players.find(p => p.id === game.black_id)}
                                game={game}
                                key={game.id}
                            />
                        )
                    }
                </tbody>
            </table>
            <table>
                <caption>Byes</caption>
                <thead>
                    <tr>
                        <th>Id</th>
                        <th>Rating</th>
                        <th>Nombre</th>
                        <th>Punto</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        byes.map((bye, i) =>
                            <ByeRow
                                bye={bye}
                                player={players.find(player => player.id === bye.player_id)}
                                key={i}
                            />
                        )
                    }
                </tbody>
            </table>
        </>
    )
}

export default Pairings
