import React from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider, createBrowserRouter, redirect } from "react-router-dom";
import Home from "./routes/Home";
import { invoke } from "@tauri-apps/api";
import "./styles.css";
import TournamentLayout from "./routes/TournamentLayout";
import TournamentData from "./routes/TournamentData";
import Players from "./routes/Players";

const router = createBrowserRouter([
  {
    index: true,
    element: <Home></Home>
  },
  {
    path: "tournament/:path",
    element: <TournamentLayout></TournamentLayout>,
    children: [
      {
        path: "data",
        loader: async ({params}) => {
          const path = atob(params.path);

          return invoke("get_tournament", {path: path})
            .then((tournament) => tournament)
            .catch((error) => {console.error(error); return redirect("/")})
        },
        element: <TournamentData></TournamentData>
      },
      {
        path: "players",
        loader: async ({params}) => {
          const path = atob(params.path)

          return invoke("get_players", {path: path})
            .then((players) => players)
            .catch((error) => {console.error(error); return redirect("/")})
        },
        element: <Players></Players>
      }
    ]
  }
])

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <RouterProvider router={router}></RouterProvider>
  </React.StrictMode>,
);
