import React, { useEffect, useState } from 'react'
import axios from 'axios'
import { useParams } from 'react-router-dom'
import { TableLink } from '../styles'

const Order = () => {
  const {table_number, id} = useParams()
  const [orderDetails, setOrderDetails] = useState({
    "tableNumber": 0,
    "item": "ITEM NOT LOADED",
    "status": 0,
  })

  async function getOrder() {
    try {
      const response = await axios.get('http://localhost:8000/api/orders/' + id)
      const data = response.data.order

      const created_at = Date.parse(data.createdAt)
      const done_at = created_at + (data.cookTime * 60000)
      if(Date.now() > done_at) {
        data.status = "Now!"
      } else {
        const time_left = Math.round((done_at - Date.now()) / 60000)
        if(time_left === 1) {
          data.status = `${time_left} min`
        } else {
          data.status = `${time_left} mins`
        }
      }

      setOrderDetails(data)
    } catch (e) {
      console.log(e)
    }
  }

  async function handleDelete(id) {
    try {
      await axios.delete('http://localhost:8000/api/orders/' + id)
      window.location.replace('/tables/' + table_number)
    } catch (e) {
      console.log(e)
    }
  }

  useEffect(() => {
    getOrder()
  }, [])

  return (
    <div>
      <h1>order page</h1>
      <TableLink to={'/tables/' + table_number}>Return</TableLink>
      <ul>
        <li>
          <b>Table Number: </b>{orderDetails.tableNumber}
        </li>
        <li>
          <b>Item: </b>{orderDetails.item}
        </li>
        <li>
          <b>Ready in approximately: </b>{orderDetails.status}
        </li>
      </ul>
      <button onClick={() => handleDelete(orderDetails.id)}>Delete</button>
    </div>
  )
}

export default Order
