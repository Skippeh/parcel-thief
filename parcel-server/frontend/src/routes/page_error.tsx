import * as React from "react";
import { isRouteErrorResponse, useRouteError } from "react-router-dom";
import { styled } from "styled-components";

const Wrapper = styled.div`
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
`;

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
    <Wrapper>
      <div>
        <strong>An unexpected error occurred.</strong>
        <p>{errorMessage}</p>
      </div>
    </Wrapper>
  );
};

export default PageError;
