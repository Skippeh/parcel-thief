import * as React from "react";
import styled from "styled-components";
import useSession from "../../hooks/use_session";

const Wrapper = styled.div`
  grid-area: 1 / 1 / 1 / 3;

  background: rgb(31, 37, 55);
  display: flex;
  justify-content: right;
  align-items: center;
  padding: 1rem;

  & .user {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    cursor: pointer;

    & .name {
      font-weight: bold;
      font-size: 0.8rem;
    }

    & .avatar {
      width: 2rem;

      & img {
        width: 100%;
        border-radius: 50%;
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
