import React  from 'react'
import { TablesContainer, TableLink } from '../styles'


const Home = () => {

  return (
    <div>
      <h1>Please select a table below to view/create/delete orders for that table:</h1>
      <TablesContainer>
        {
          Array.from({ length: 100 }).map((_, i) => <TableLink key={i} to={'/tables/'+(i+1)} activeStyle>Table #{i+1}</TableLink>)
        }
      </TablesContainer>
    </div>
  )
}

export default Home
