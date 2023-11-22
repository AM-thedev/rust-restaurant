import React, { useEffect, useState } from 'react'
import axios from 'axios'
import {
  useParams,
  NavLink
} from "react-router-dom"
import { TableLink, OrdersTable } from '../styles'
import Modal from '../components/Modal';


const Table = () => {
  let { table_number } = useParams()

  const [ordersList, setOrdersList] = useState([])
  const [modal, setModal] = useState(false)

  async function getOrders() {
    try {
      const response = await axios.get('http://localhost:8000/api/tables/' + table_number)
      const data = response.data.orders
      setOrdersList(data)
    } catch (e) {
      console.log(e)
    }
  }

  useEffect(() => {
    getOrders()
  }, [])

  const toggle = () => {
    setModal(!modal)
  }

  const getTimeLeft = (createdAt, cookTime) => {
    const created_at = Date.parse(createdAt)
    const done_at = created_at + (cookTime * 60000)
    let status = ""
    if (Date.now() > done_at) {
      status = "Now!"
    } else {
      const time_left = Math.round((done_at - Date.now()) / 60000)
      if (time_left === 1) {
        status = `${time_left} min`
      } else {
        status = `${time_left} mins`
      }
    }
    return status
  }

  async function handleCreate(orders) {
    toggle()

    try {
      await axios.post('http://localhost:8000/api/tables/' + table_number, orders)
      getOrders()
    } catch (e) {
      console.log(e)
    }
  }

  async function handleDelete(id) {
    try {
      await axios.delete('http://localhost:8000/api/orders/' + id)
      getOrders()
    } catch (e) {
      console.log(e)
    }
  }

  const createOrders = () => {
    setModal(!modal)
  }

  const renderOrders = (
    <tbody>
      {ordersList.map(order => (
        <tr key={order.id}>
          <td>{order.item}</td>
          <td>{getTimeLeft(order.createdAt, order.cookTime)}</td>
          <td><NavLink to={'/tables/' + table_number + '/' + order.id}>Details</NavLink></td>
          <td><button onClick={() => handleDelete(order.id)}>Delete</button></td>
        </tr>
      ))}
    </tbody>
  )

  return (
    <div>
      <h1>table page</h1>
      <TableLink to={'/'}>Return</TableLink>
      <button onClick={createOrders}>Create New Orders</button>
      <OrdersTable>
        <thead>
          <tr>
            <th>Item</th>
            <th>Ready in approximately</th>
            <th>Details</th>
            <th>Delete</th>
          </tr>
        </thead>
        {renderOrders}
      </OrdersTable>
      {modal ? (
          <Modal
            toggle={toggle}
            onSave={handleCreate}
          />
        ): null}
    </div>
  )
}

export default Table
