import { useRouteError } from "react-router-dom";

export default function ErrorPage() {
  const error = useRouteError();
  console.error(error);

  return (
    <div id="error-page">
      <h1>{error.statusText || error.message}</h1>
      <p>thismachinewillnotcommunicate</p>
    </div>
  );
}
