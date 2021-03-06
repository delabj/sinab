#' Render HTML or Markdown
#' 
#' Render HTML or Markdown. The function `html_grob()` returns a grid
#' grob that can be drawn with [grid.draw()], and the function
#' `draw_html()` renders the HTML directly to the current graphics
#' device.
#' @param text HTML or Markdown text to render
#' @param x,y x and y position
#' @param width,height Width and height. A value of `NULL` means
#'   take up the available space as needed.
#' @param hjust,vjust horizontal and vertical justification relative
#'  to `x` and `y`
#' @param css CSS specification to use for rendering
#' @param default.units	Units of `x`, `y`, `width`, `height` if
#'  these are provided only as numerical values.
#' @param vp viewport
#' @return `html_grob()`: A grid grob representing the rendered html text.
#' @export
html_grob <- function(text, x = unit(0.05, "npc"), y = unit(0.95, "npc"),
                      width = unit(0.9, "npc"), height = NULL,
                      hjust = 0, vjust = 1, css = "", 
                      default.units = "npc", vp = NULL) {
  # make sure x, y, width, height are units
  x <- with_unit(x, default.units)
  y <- with_unit(y, default.units)
  width <- with_unit(width, default.units)
  height <- with_unit(height, default.units)
  
  # make sure we can handle input text even if provided as factor
  text <- as.character(text)
  # convert NAs to empty strings
  text <- ifelse(is.na(text), "", text)
  
  # if x or y are NULL, take from hjust, vjust
  if (is.null(x)) {
    x <- unit(hjust, "npc")
  }
  if (is.null(y)) {
    y <- unit(vjust, "npc")
  }
  
  gTree(
    text = text,
    x = x,
    y = y,
    width = width,
    height = height,
    hjust = hjust,
    vjust = vjust,
    css = css,
    vp = vp,
    cl = "html_grob"
  )
}

#' @rdname html_grob
#' @param ... Parameters forwarded to `html_grob()`.
#' @param newpage Logical indicating whether [grid.newpage()] should
#'   be called before drawing.
#' @return `draw_html()`: Nothing. The function is only called for its side effects.
#' @export
draw_html <- function(..., newpage = TRUE) {
  if (isTRUE(newpage)) {
    grid.newpage()
  }
  grid.draw(html_grob(...))
  invisible()
}


#' @export
makeContext.html_grob <- function(x) {
  x$width_inch <- current_width(x, x$width)
  # For now, we're using the specified height,
  # not the calculated height
  x$height_inch <- current_height(x, x$height)
  
  # Create the children. This would normally be
  # done in `makeContent()`, but we need to do it
  # here to capture the output dimensions. 
  children <- render_markdown(x$text, x$css, x$width_inch, x$height_inch)
  
  # record bbox width and height for widths and heights that aren't set
  bbox <- attributes(children)$bbox
  if (!is.null(bbox)) {
    if (is.null(x$width)) {
      x$width_inch <- bbox$xmax - bbox$xmin
    }
    if (is.null(x$height)) {
      x$height_inch <- bbox$ymax - bbox$ymin
    }
  }

  vp <- viewport(x$x, x$y, just = c(x$hjust, 1-x$vjust))
  if (is.null(x$vp)) {
    x$vp <- vp
  } else {
    x$vp <- vpStack(x$vp, vp)
  }
  setChildren(x, children)
}


#' @export
heightDetails.html_grob <- function(x) {
  unit(x$height_inch, "inches")
}

#' @export
widthDetails.html_grob <- function(x) {
  unit(x$width_inch, "inches")
}

#' @export
ascentDetails.html_grob <- function(x) {
  unit(x$height_inch, "inches")
}

#' @export
descentDetails.html_grob <- function(x) {
  unit(0, "inches")
}