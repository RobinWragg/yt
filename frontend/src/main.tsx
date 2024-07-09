// No content. Just specifies routes.

import React from "react";
import ReactDOM from "react-dom/client";
import Root from "./root.tsx";
import ErrorPage from "./error_page.tsx";
import Yt from "./yt.tsx";
import "./index.css";
import { createBrowserRouter, RouterProvider } from "react-router-dom";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Root />,
    errorElement: <ErrorPage />,
  },
  {
    path: "yt",
    element: <Yt />,
  },
]);

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);
