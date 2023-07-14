import * as React from "react";
import { User } from "../../context/session_context";
import styled from "styled-components";

const Wrapper = styled.div`
  padding: 0 1rem;
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
      border: 1px solid #2ed9d9;
      box-shadow: 0 0 1px 0px #2ed9d9 inset, 0 0 1px 0px #2ed9d9;
    }
  }
`;

interface Props {
  user: User;
}

const User = ({ user }: Props) => {
  return (
    <Wrapper>
      <div className="name">{user.name}</div>
      <div className="avatar">
        <img src={user.avatarUrl} />
      </div>
    </Wrapper>
  );
};

export default User;
