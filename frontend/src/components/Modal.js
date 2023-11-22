import React from "react"
import 'bootstrap/dist/css/bootstrap.min.css'
import { Formik, Field, Form, ErrorMessage, FieldArray } from 'formik'
import {
  Modal,
  ModalHeader,
  ModalBody,
  ModalFooter,
} from "reactstrap"


export default function AlbumModal({ toggle, onSave }) {

  const initialValues = {
    orders: [
      {
        item: '',
        cook_time: 15,
      },
    ],
  }

  return (
    <Modal isOpen={true} toggle={toggle}>
      <ModalHeader toggle={toggle}>Order</ModalHeader>
      <ModalBody>
        <Formik
          initialValues={initialValues}
          onSubmit={async (values) => {
            await new Promise((r) => setTimeout(r, 500))
            onSave(values)
          }}
        >
          {({ values }) => (
            <Form>
              <FieldArray name="orders">
                {({ insert, remove, push }) => (
                  <div>
                    {values.orders.length > 0 &&
                      values.orders.map((order, index) => (
                        <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }} key={index}>
                          <div className="col">
                            <label htmlFor={`orders.${index}.item`}>Item</label>
                            <br></br>
                            <Field
                              name={`orders.${index}.item`}
                              placeholder="Enter item name here"
                              type="text"
                            />
                            <ErrorMessage
                              name={`orders.${index}.item`}
                              component="div"
                              className="field-error"
                            />
                          </div>
                          <div className="col">
                            <label htmlFor={`orders.${index}.cook_time`}>Cook time between 1-30 minutes</label>
                            <br></br>
                            <Field
                              name={`orders.${index}.cook_time`}
                              type="number"
                              min="1"
                              max="30"
                            />
                            <ErrorMessage
                              name={`orders.${index}.name`}
                              component="div"
                              className="field-error"
                            />
                          </div>
                          {
                            index > 0 ? (
                              <div className="col">
                                <button
                                  type="button"
                                  className="secondary"
                                  onClick={() => remove(index)}
                                >
                                  Remove order
                                </button>
                              </div>
                            ) : null
                          }
                          <br></br>
                        </div>
                      ))}
                    <button
                      type="button"
                      className="secondary"
                      onClick={() => push({ item: '', cook_time: 15 })}
                    >
                      Add Order
                    </button>
                  </div>
                )}
              </FieldArray>
              <button type="submit">Submit</button>
            </Form>
          )}
        </Formik>
      </ModalBody>
      <ModalFooter>
        {/*
        <Button color="success" onClick={() => onSave(orderForm)}>
          Submit
        </Button>
                      */}
      </ModalFooter>
    </Modal>
  )
}