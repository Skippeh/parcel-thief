import * as React from "react";
import { Link, useMatches } from "react-router-dom";
import styled from "styled-components";
import { RouteHandle } from "../routes";

const Wrapper = styled.div`
  & ul {
    display: flex;
    list-style: none;
    padding: 0;
    margin: 0;
    gap: 0.5rem;
    font-size: 0.9rem;
    align-items: center;

    & > li:not(:last-child) {
      display: inline-block;
    }

    & .separator {
      margin-left: 0.5rem;
    }
  }
`;

const Breadcrumb = () => {
  const matches = useMatches().filter((m) => m.handle != null);

  // don't render breadcrumb if we're on the home page
  if (matches.length == 1) {
    return null;
  }

  return (
    <Wrapper>
      <ul>
        {matches.map((m, index) => {
          const handle = m.handle as RouteHandle;
          return (
            <li key={m.id}>
              <Link to={m.pathname}>{handle.crumb}</Link>

              {index < matches.length - 1 && (
                <span className="separator">/</span>
              )}
            </li>
          );
        })}
      </ul>
    </Wrapper>
  );
};

export default Breadcrumb;
