import React from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider, createBrowserRouter } from "react-router-dom";
import Home from "./routes/Home";
import TournamentLayout from "./routes/TournamentLayout";
import TournamentData from "./routes/TournamentData";
import Players from "./routes/Players";
import Pairings from "./routes/Pairings";
import Standings from "./routes/Standings";
import ErrorRoute from "./routes/ErrorRoute";

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
        index: true,
        element: <TournamentData></TournamentData>
      },
      {
        path: "players",
        element: <Players></Players>
      },
      {
        path: ":round",
        children: [
          {
            path: "pairings",
            element: <Pairings></Pairings>
          },
          {
            path: "standings",
            element: <Standings></Standings>
          }
        ]
      }
    ]
  },
  {
    path: "error",
    element: <ErrorRoute></ErrorRoute>
  }
])

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <RouterProvider router={router}></RouterProvider>
  </React.StrictMode>,
);
