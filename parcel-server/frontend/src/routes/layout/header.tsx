import * as React from "react";
import styled from "styled-components";
import useSession from "../../hooks/use_session";
import { NavLink } from "react-router-dom";

const Wrapper = styled.div`
  grid-area: 1 / 1 / 1 / 3;

  background: rgb(31, 37, 55);
  display: flex;
  justify-content: space-between;
  align-items: center;

  & .user {
    padding: 1rem;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    cursor: pointer;

    & .name {
      font-weight: bold;
      font-size: 0.8rem;
    }

    & .avatar {
      height: 2rem;
      width: 2rem;

      & img {
        width: 100%;
        border-radius: 50%;
      }
    }
  }

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
            background: #257524;
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
      <nav>
        <ul>
          <li>
            <NavLink to="/">Home</NavLink>
          </li>
        </ul>
      </nav>
      <div className="user">
        <div className="name">{user.name}</div>
        <div className="avatar">
          <img src={user.avatarUrl} />
        </div>
      </div>
    </Wrapper>
  );
};

export default Header;
