import * as Colors from "@radix-ui/colors";
import styled from "styled-components";

export const TableWrapper = styled.div.attrs({
  className: "ag-theme-alpine-dark",
})`
  & .ag-row .buttons {
    opacity: 0.5;
    transition: opacity 0.1s ease-out;
  }

  & .ag-row:hover .buttons {
    transition: none;
    opacity: 1;
  }
`;

export const TableButtons = styled.div.attrs({
  className: "buttons",
})`
  & a {
    color: ${Colors.whiteA.whiteA12};
    font-size: 1.3rem;
    vertical-align: middle;
  }
`;
