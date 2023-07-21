import { RootState } from '@/app/store'
import Button from '@/components/Button'
import { setSidebarWidth, toggleSidebar } from '@/features/appSlice'
import { isInView } from '@/hooks/selectors'
import { ChevronDoubleLeftIcon } from '@heroicons/react/24/outline'
import { useCallback, useEffect, useState } from 'react'
import { IoMdJournal } from 'react-icons/io'
import { useDispatch, useSelector } from 'react-redux'
import SidebarGroup from './sidebar-group'
import './sidebar.css'
import { pagesDummy } from '@/utils/constants'
import SidebarExpandable from './sidebar-expandable'
const Sidebar = () => {
  const sidebarWidth = useSelector(
    ({ app }: RootState) => app.sidebar.sidebarWidth
  )
  const { sidebar: showSidebar } = useSelector(isInView)

  const dispatch = useDispatch()

  const [isDragging, setIsDragging] = useState(false)

  const handleMouseDown = (ev: React.MouseEvent<HTMLDivElement>) => {
    ev.preventDefault()
    // document.documentElement.style.cursor = 'w-resize'
    setIsDragging(true)
  }

  const handleMouseUp = () => {
    setIsDragging(false)
    // document.documentElement.style.cursor = 'default'
  }

  const handleToggleSidebar = () => {
    dispatch(toggleSidebar())
  }
  const handleMouseMove = useCallback(
    (ev: MouseEvent) => {
      if (isDragging) {
        const minSidebarWidth = 270
        const maxSidebarWidth = 400
        const newWidth = Math.max(
          minSidebarWidth,
          Math.min(maxSidebarWidth, ev.pageX)
        )
        dispatch(setSidebarWidth(newWidth))
      }
    },
    [dispatch, isDragging]
  )

  useEffect(() => {
    document.addEventListener('mousemove', handleMouseMove)
    document.addEventListener('mouseup', handleMouseUp)

    return () => {
      document.removeEventListener('mousemove', handleMouseMove)
      document.removeEventListener('mouseup', handleMouseUp)
    }
  }, [handleMouseMove, handleMouseUp])

  return (
    <div
      className={`main__sidebar ${isDragging ? 'dragging' : ''} ${
        showSidebar ? '' : 'hidden'
      }`}
      style={{ width: sidebarWidth }}
    >
      <div className="main__sidebar__modern">
        <header className="sidebar-modern__header">
          <div className="sidebar-modern__header__left">
            <IoMdJournal />
            <span>My Notebook</span>
          </div>
          <div className="sidebar-modern__header__right">
            <Button onClick={handleToggleSidebar} variant={'transparent'}>
              <ChevronDoubleLeftIcon width={19} />
            </Button>
          </div>
        </header>

        <section className="sidebar__main">
          <SidebarGroup groupTitle="Pages">
            {pagesDummy.map((page) => {
              return <SidebarExpandable key={page._id_} page={page} />
            })}
          </SidebarGroup>
        </section>
      </div>
      <div className="resizer" onMouseDown={handleMouseDown}></div>
    </div>
  )
}

export default Sidebar