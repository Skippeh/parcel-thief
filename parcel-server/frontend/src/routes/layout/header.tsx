import * as React from "react";
import styled from "styled-components";
import useSession from "../../hooks/use_session";
import { NavLink } from "react-router-dom";
import User from "./user";
import * as Colors from "@radix-ui/colors";
import { CenterContainer } from ".";
import ProtectedContent from "../protected_content";

const Wrapper = styled.div`
  background: ${Colors.blueDark.blue3};
  border-bottom: 1px solid ${Colors.blueDark.blue4};
  box-shadow: 0px 14px 36px -13px rgba(0, 0, 0, 0.3);

  display: flex;
  justify-content: center;
`;

const InnerContainer = styled.div`
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
          color: ${Colors.whiteA.whiteA12};
          padding: 1rem;
          transition: background-color 0.1s ease-out;

          &.active {
            background: ${Colors.blueDark.blue8};
          }

          &:not(.active):hover {
            background: ${Colors.blueDark.blue7};
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
        <InnerContainer>
          <nav>
            <ul>
              <li>
                <NavLink to="/">Home</NavLink>
              </li>
              <li>
                <NavLink to="/items">Items</NavLink>
              </li>
              <ProtectedContent permissions={["manageAccounts"]}>
                <li>
                  <NavLink to="/accounts">Accounts</NavLink>
                </li>
              </ProtectedContent>
            </ul>
          </nav>
          <User user={user} />
        </InnerContainer>
      </CenterContainer>
    </Wrapper>
  );
};

export default Header;
