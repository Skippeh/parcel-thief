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
    color: ${Colors.whiteA.whiteA12};
    background: rgb(2, 0, 36);
    background: radial-gradient(
      circle at center top,
      rgba(24, 27, 52, 1) 0%,
      rgba(2, 0, 36, 1) 100%
    );

    & a {
      color: ${Colors.indigoDark.indigo11};
      text-decoration: none;
      transition: color 0.05s ease-out;

      &:hover {
        color: hsl(228 100% 85.9%);
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
  }

  button, input[type=submit], .button {
    all: unset;
    display: inline-block;
    box-sizing: border-box;
    margin: 0.25rem;
    padding: 0.6rem 0.9rem;
    background: ${Colors.blueDark.blue7};
    text-align: center;
    border-radius: 2px;
    user-select: none;
    transition: background-color 0.1s ease-out;

    
  }

  button, input[type=submit], .button {
    &, &.primary {
      background: ${Colors.blueDark.blue8};

      &:hover, &:focus {
        background: ${Colors.indigoDark.indigo9};
        color: inherit;
      }
    }
  }
`;
