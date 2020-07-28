#ifndef RENDERER_H
#define RENDERER_H 

#ifdef __cplusplus
extern "C" {
#endif

#include <R.h>
#include <Rinternals.h>
#include <R_ext/GraphicsEngine.h>

/* 
 * Grid renderer object.
 */

typedef struct {
  SEXP grobs;         /* vector holding the rendered grobs */
  R_xlen_t size;      /* current number of rendered grobs */
  R_xlen_t capacity;  /* capacity of the grobs vector; always >= size */
  double y0;          /* y reference value, used to invert coordinate system */
} RenderDevice;

/* 
 * Graphics context. Similar to R_GE_gcontext
 * from R_ext/GraphicsEngine.h.
 */

#define STRING_BUFFER_SIZE 201 /* 201 is number used by R */

typedef struct {
  char color[STRING_BUFFER_SIZE];        /* Drawing color: lines, text, etc. */
  char fill[STRING_BUFFER_SIZE];         /* Fill color */
  double fontsize;       /* Font size in points (R_GE_gcontext.ps) */
  double lineheight;     /* Line height (in multiples of fontsize) */
  int fontface;          /* Font face:
                          *  plain = 1, bold = 2,
                          *  italic = 3, bold italic = 4 
                          */
  char fontfamily[STRING_BUFFER_SIZE];  /* Font family */
} GContext;


/*
 * Function declarations
 */

/* render_device.c */
extern RenderDevice* rdev_new(double y0);
extern SEXP rdev_release(RenderDevice*); 
extern void rdev_draw_text(RenderDevice*, const char* label, double x, double y, const GContext *);
extern void rdev_draw_rect(RenderDevice*, double x, double y, double width, double height, const GContext *);
extern void rdev_string_metrics(const char* label, const GContext *,
                                double *ascent, double *descent, double *width);
double rdev_device_height();

extern SEXP test_rdev_new_release(SEXP);
extern SEXP test_gpar_gcontext();


/* gcontext.c */

extern GContext* gcontext_new();
extern GContext* gcontext_clone(GContext*);
extern void gcontext_delete(GContext*);
extern void gcontext_set_color(GContext*, const char*);
extern const char* gcontext_color(GContext*);
extern void gcontext_set_fill(GContext*, const char*);
extern const char* gcontext_fill(GContext*);
extern void gcontext_set_fontfamily(GContext*, const char*);
extern const char* gcontext_fontfamily(GContext*);
extern void gcontext_set_fontface(GContext*, int);
extern int gcontext_fontface(GContext*);
extern void gcontext_set_fontsize(GContext*, double);
extern double gcontext_fontsize(GContext*);
extern void gcontext_set_lineheight(GContext*, double);
extern double gcontext_lineheight(GContext*);

/* r-callbacks.c */
extern SEXP text_grob(SEXP, SEXP, SEXP, SEXP, SEXP, SEXP);
extern SEXP rect_grob(SEXP, SEXP, SEXP, SEXP, SEXP, SEXP, SEXP);
extern SEXP gpar_empty();
extern SEXP gpar_gcontext(const GContext *);
extern SEXP unit_in(SEXP);

#ifdef __cplusplus
}
#endif

#endif
