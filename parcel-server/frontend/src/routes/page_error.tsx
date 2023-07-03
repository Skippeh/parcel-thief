import * as React from "react";
import { isRouteErrorResponse, useRouteError } from "react-router-dom";

const PageError = () => {
  const error = useRouteError();
  console.error("Page error", error);

  let errorMessage;

  if (isRouteErrorResponse(error)) {
    if (error.status === 404) {
      errorMessage = "Page not found";
    } else {
      errorMessage = error.error?.message || error.statusText;
    }
  } else if (error instanceof Error) {
    errorMessage = error.message;
  } else if (typeof error == "string") {
    errorMessage = error;
  } else {
    errorMessage = "Unknown error, check console for more information";
  }

  return (
    <div>
      <p>An unexpected error occurred.</p>
      <p>{errorMessage}</p>
    </div>
  );
};

export default PageError;
