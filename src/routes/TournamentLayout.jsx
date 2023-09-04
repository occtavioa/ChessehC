import { useState } from "react";
import { Link, Outlet, useLoaderData } from "react-router-dom"

function TournamentLayout() {
    const currentRound = useLoaderData();
    const [selectedRound, setSelectedRound] = useState(currentRound)

    return (
        <>
            <nav>
                <Link to={`/`}>Inicio</Link>
                <Link to={`.`}>Torneo</Link>
                <Link to={`${selectedRound}/players`}>Jugadores</Link>
                <Link to={`${selectedRound}/pairings`}>Pareos</Link>
            </nav>
            {
                Number.isInteger(currentRound) ?
                    <select onChange={(e) => {
                        setSelectedRound(e.target.value)
                    }}>
                        {(new Array(currentRound)).map((r) => <option value={r}>Ronda {r}</option>)}
                    </select> :
                    <></>
            }
            <Outlet></Outlet>
        </>
    )
}

export default TournamentLayout;
