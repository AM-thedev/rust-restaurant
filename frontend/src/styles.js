import styled from "styled-components";
import { NavLink } from 'react-router-dom';

export const TablesContainer = styled.div`
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}`;

export const TableLink = styled(NavLink)`
  background-color: ghostwhite;
  border-color: gray;
  border-width: 2px;
  padding: 4px;
  border-style: solid;
  border-radius: 5px;
  white-space: nowrap;
  text-decoration: none;
}`;

export const OrdersTable = styled.table`
  width: 100%;
  border-collapse: collapse;
  margin-top: 16px;

  td, th {
    border: 1px solid #dddddd;
    text-align: left;
    padding: 8px;
  }
  
  tr:nth-child(even) {
    background-color: #dddddd;
  }
}`;