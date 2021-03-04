import React from "react";

import { Outlet } from 'react-router-dom';
import Header from "../components/Header";

import styled from "styled-components";

const Content = styled.div`
  .container {
    padding: 15px;
  }
`;

function Main(): React.ReactElement {
  return (
    <>
      <Header />
      <Content>
        <div className="container">
          <Outlet />
        </div>
      </Content>
    </>
  );
}

export default React.memo(Main);