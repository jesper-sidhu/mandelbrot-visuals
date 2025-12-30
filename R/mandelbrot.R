#' Core mandelbrot Algorithm
#' The mandelbrot iteration:
#' for each complex number c, we start with z0 = 0 and repeatedly apply:
#'   z(n + 1) = z(n)^2 + c
#' if this sequence stays bounded (|z| <= 2), then c is in the mandelbrot set

mandelbrot <- function(
    c,
    max_iter = 100
) {
    z <- 0
    for (i in seq_len(max_iter)) {
        z <- z^2 + c
        if (abs(z) > 2) {
            return()
        }
    }

    return(max_iter)
}


#' Visualisation function

plot_mandelbrot <- function(
    centre_x = -0.5,
    centre_y = -0,
    zoom = 1,
    width = 800,
    height = 600
) {
    # Calculate Viewing window
    scale <- 3.5 / zoom
    x_min <- centre_x - scale * width / (2 * height)
    x_max <- centre_x + scale * width / (2 * height)
    y_min <- centre_y - scale / 2
    y_max <- centre_y + scale / 2

    # Create coordinate grid
    x <- seq(x_min, x_max, length.out = width)
    y <- seq(y_min, y_max, length.out = height)

    # Calculate iterations for each point
    max_iter <- min(100 + floor(zoom * 20), 500)
    m <- outer(
        x,
        y,
        function(real, imaginary) {
            sapply(real + 1i * imaginary, mandelbrot, max_iter = max_iter)
        }
    )

    # Create colour palette
    colours <- colorRampPalette(
        c(
            "#000033",
            "#000055",
            "#0000BB",
            "#5500BB",
            "#BB00BB",
            "#FF0055",
            "#FF5500",
            "#FFBB00",
            "#FFFF00"
        )
    )(max_iter)

    # Plot the Mandelbrot set
    par(mar = c(3, 3, 3, 1))
    image(
        x,
        y,
        m,
        col = colours,
        useRaster = TRUE,
        main = sprintf("Mandelbrot Set (Zoom: %.1fx)", zoom),
        xlab = sprintf("Real (centre: %.6f)", centre_x),
        ylab = sprintf("Imaginary (centre: %.6f)", centre_y),
    )

    list(
        centre_x = centre_x,
        centre_y = centre_y,
        zoom = zoom,
        x_range = c(x_min, x_max),
        y_range = c(y_min, y_max)
    )
}


#' Zoom function

zoom_loop <- function() {
    view <- list(centre_x = -0.5, centre_y = 0, zoom = 1)

    cat("Interactive Mandelbrot Explorer\n")
    cat("Click anywhere to zoom in 2x\n")
    cat("Press ESC to exit\n\n")

    repeat {
        view_params <-
            plot_mandelbrot(
                view$centre_x,
                view$centre_y,
                view$zoom
            )

        click <- locator(1)

        if (is.null(click)) {
            break
        }

        view$centre_x <- click$x
        view$centre_y <- click$y
        view$zoom <- view$zoom * 2

        cat(sprintf(
            "Zooming to (%.6f, %.6f) at %.1fx\n",
            view$centre_x,
            view$centre_y,
            view$zoom
        ))
    }
    cat("Exiting...\n")
}
