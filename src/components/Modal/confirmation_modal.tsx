import { useNebulaCore } from '@/context/nebula'
import { IConfirmationModal, NebulaModal } from '@/features/modalSlice'
import React, { useCallback, useEffect, useState } from 'react'
import { AiOutlineWarning } from 'react-icons/ai'
import { CgDanger } from 'react-icons/cg'
import { useDispatch } from 'react-redux'
import Button from '../Button'
import './confirmation_modal.css'

type Props = {
  modal: IConfirmationModal
}
const ConfirmationModal: React.FC<Props> = ({ modal }) => {
  const dispatch = useDispatch()
  const [submitTimeLeft, setSubmitTimeLeft] = useState(
    modal.dangerLevel > 1 ? 5 : -1
  )

  const nebula = useNebulaCore()
  const getConfirmationHandler = useCallback(
    (modal: IConfirmationModal) => {
      switch (modal.id) {
        case 'removePage': {
          return () => {
            if (modal.props.type === 'removePage') {
              nebula.core.movePageToTrash(modal.props.pageId)
            }
          }
        }
        case 'removePagePermanent': {
          return () => {
            if (modal.props.type === 'removePagePermanent') {
              nebula.core.deletePagePermanent(modal.props.pageId)
            }
          }
        }
      }
    },
    [dispatch, modal]
  )
  useEffect(() => {
    let interval: NodeJS.Timer
    if (modal.dangerLevel > 1) {
      interval = setInterval(() => {
        if (submitTimeLeft === -1) {
          interval !== undefined && clearInterval(interval)
          return
        }
        setSubmitTimeLeft((prev) => prev - 1)
      }, 1000)
    }
    return () => {
      interval !== undefined && clearTimeout(interval)
    }
  }, [])

  return (
    <div className="modal__confirmation_modal_container">
      <div
        className="modal__confirmation_modal"
        onClick={(ev) => ev.stopPropagation()}
      >
        <header>
          <div className="heading">
            {modal.dangerLevel === 1 && (
              <AiOutlineWarning style={{ color: 'yellow' }} />
            )}
            {modal.dangerLevel === 2 && <CgDanger style={{ color: 'red' }} />}
            <h3>{modal.for}</h3>
          </div>
          <p>{modal.information}</p>
        </header>
        <div className="confirmation_modal__actions">
          <Button
            onClick={getConfirmationHandler(modal)}
            disabled={submitTimeLeft > -1}
            className={`modal__confirmation_modal_btn confirm ${
              modal.dangerLevel > 1 ? 'danger' : 'warning'
            }`}
          >
            {submitTimeLeft < 0 ? 'Remove' : 'Wait ' + submitTimeLeft + ' (s)'}
          </Button>
          <Button
            onClick={() => dispatch(NebulaModal.unloadModal())}
            className="modal__confirmation_modal_btn reject"
            autoFocus
          >
            Cancel
          </Button>
        </div>
      </div>
    </div>
  )
}

export default ConfirmationModal
