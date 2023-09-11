import { Link } from "react-router-dom"

function ErrorRoute() {
    return (
        <>
            <Link to={"/"}>Volver al inicio</Link>
            <p>Hubo un error</p>
        </>
    )
}

export default ErrorRoute
