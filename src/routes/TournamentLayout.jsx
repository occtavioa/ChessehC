import { Link, Outlet } from "react-router-dom"

function TournamentLayout() {
    return (
        <>
            <nav>
                <Link to={`/`}>Inicio</Link>
                <Link to={`.`}>Torneo</Link>
                <Link to={`players`}>Jugadores</Link>
                <Link to={`standings`}>Posiciones</Link>
                <Link to={`pairings`}>Pareos</Link>
            </nav>
            <Outlet></Outlet>
        </>
    )
}

export default TournamentLayout;
