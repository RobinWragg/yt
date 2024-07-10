import { useRouteError } from "react-router-dom";

export default function ErrorPage() {
  const error = useRouteError();
  console.error(error);
  console.error(typeof error);

  function errorMessage(error: unknown) {
    if (error instanceof Error) {
      return error.message;
    } else if (
      error instanceof Object &&
      "statusText" in error &&
      typeof error.statusText === "string"
    ) {
      return error.statusText;
    } else {
      return "Unknown Error";
    }
  }

  return (
    <div id="error-page">
      <h1>{errorMessage(error)}</h1>
      <p>thismachinewillnotcommunicate</p>
    </div>
  );
}
