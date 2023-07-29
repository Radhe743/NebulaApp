import './editor.css'
import { useEffect } from 'react'
import TopBar from '@/components/topbar'
import Main from './main'
import { useParams } from 'react-router-dom'
import { useDispatch, useSelector } from 'react-redux'
import { loadNotebook, loadPage } from '@/features/editorReducers'
import { AppDispatch, RootState } from '@/app/store'
import { resetNotebookState, unloadPage } from '@/features/editorSlice'

import useUpdatePage from '@/hooks/use-update-page'
const Editor = () => {
  const params = useParams()
  const dispatch = useDispatch<AppDispatch>()
  const currentNotebook = useSelector(
    (state: RootState) => state.editor.currentNotebook
  )
  const currentPage = useSelector(
    (state: RootState) => state.editor.currentPage
  )
  const notebookId = params.notebook
  const pageId = params.pageId
  useUpdatePage(2000)
  useEffect(() => {
    if (notebookId !== undefined) {
      dispatch(loadNotebook({ notebookId: notebookId }))
    }

    return () => {
      console.log('Unload')
      dispatch(resetNotebookState())
    }
  }, [dispatch, notebookId])

  useEffect(() => {
    if (pageId !== undefined) {
      dispatch(loadPage({ pageId }))
    } else {
      dispatch(unloadPage())
    }
  }, [dispatch, pageId])

  return (
    <div className="app__editor">
      <TopBar />
      {currentNotebook ? <Main /> : <div>Notebook Not found</div>}
    </div>
  )
}

export default Editor
