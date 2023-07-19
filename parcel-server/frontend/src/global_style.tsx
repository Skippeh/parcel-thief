import { createGlobalStyle } from "styled-components";
import * as Colors from "@radix-ui/colors";

export default createGlobalStyle`
  html,
  body,
  #root {
    height: 100%;
  }

  * {
    box-sizing: border-box;
  }

  body {
    font-family: "Open Sans";
    font-size: 0.9rem;
    color: ${Colors.whiteA.whiteA12};
    background: rgb(2, 0, 36);
    background: radial-gradient(
      circle at center top,
      rgba(24, 27, 52, 1) 0%,
      rgba(2, 0, 36, 1) 100%
    );

    & a {
      color: ${Colors.skyDark.sky11};
      text-decoration: none;
      transition: color 0.05s ease-out;

      &:hover {
        color: ${Colors.skyDark.sky12};
      }
    }
  }

  input[type=text],
  input[type=password],
  textarea {
    background: ${Colors.blueDark.blue5};
    color: ${Colors.whiteA.whiteA12};
    border: 1px solid ${Colors.whiteA.whiteA8};
    border-radius: 2px;
    margin: 0.25rem 0;
    padding: 0.5rem;
    font-size: 0.9rem;

    &[data-invalid=true] {
      border-color: ${Colors.redDark.red9};
    }

    &:focus {
      outline: 1px solid ${Colors.indigoDark.indigo11};
    }

    &:read-only {
      cursor: not-allowed;
      color: ${Colors.whiteA.whiteA10};
    }
  }

  button, input[type=submit], .button {
    all: unset;
    font-size: 0.9rem;
    display: inline-block;
    box-sizing: border-box;
    margin: 0.25rem;
    padding: 0.6rem 0.9rem;
    background: ${Colors.blueDark.blue7};
    text-align: center;
    border-radius: 2px;
    user-select: none;
    transition: background-color 0.1s ease-out;

    &:hover, &:focus {
      color: inherit;
    }

    &, &.primary {
      background: ${Colors.blueDark.blue8};

      &:hover, &:focus-visible {
        background: ${Colors.indigoDark.indigo9};
      }

      &:active {
        background: ${Colors.indigoDark.indigo8};
      }

      &:disabled {
        cursor: not-allowed;
        background: ${Colors.blueDark.blue7};
        color: ${Colors.whiteA.whiteA9};

        &:hover, &:focus-visible {
          background: ${Colors.blueDark.blue7};
        }
      }
    }
  }

  .spin {
    animation: spin 1.5s linear infinite;
  }

  span {
    &.error {
      color: ${Colors.redDark.red9};
    }
  }
`;
