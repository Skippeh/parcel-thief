import * as React from "react";
import styled from "styled-components";
import useSession from "../../hooks/use_session";
import { NavLink } from "react-router-dom";
import User from "./user";

const Wrapper = styled.div`
  grid-area: 1 / 1 / 1 / 3;

  background: rgb(31, 37, 55);
  border-bottom: 1px solid rgb(36, 45, 70);
  box-shadow: 0px 14px 36px -13px rgba(0, 0, 0, 0.3);

  display: flex;
  justify-content: center;
`;

const CenterContainer = styled.div`
  width: 100%;
  height: 100%;
  max-width: 1440px;

  display: flex;
  justify-content: space-between;
  align-items: center;

  & nav {
    padding: 0 1rem;

    & ul {
      list-style: none;
      padding: 0;

      & li {
        display: inline-block;

        & a {
          text-decoration: none;
          color: #f5f5f5;
          padding: 1rem;

          &.active {
            background: #244475;
          }

          &:not(.active):hover {
            background: rgba(255, 255, 255, 0.1);
          }
        }
      }
    }
  }
`;

const Header = () => {
  const { getUser } = useSession();
  const user = getUser();

  if (user == null) return null;

  return (
    <Wrapper>
      <CenterContainer>
        <nav>
          <ul>
            <li>
              <NavLink to="/">Home</NavLink>
            </li>
            <li>
              <NavLink to="/items">Items</NavLink>
            </li>
          </ul>
        </nav>
        <User user={user} />
      </CenterContainer>
    </Wrapper>
  );
};

export default Header;
