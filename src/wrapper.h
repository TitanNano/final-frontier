#if defined(__APPLE__)
#define GL_SILENCE_DEPRECATION
#include <OpenGL/gl.h>
#include <OpenGL/glu.h>
#else
#include <GL/gl.h>
#include <GL/glu.h>
#endif

#include "../m68000.h"

#ifndef CALLBACK
# ifdef WIN32
#  define CALLBACK __attribute__ ((__stdcall__))
# else
#  define CALLBACK
# endif
#endif /* CALLBACK */

// Apple's glu.h defines its function pointers a little differently, so this tries to fix things up:
#ifdef __APPLE__
#ifndef GLAPIENTRY
typedef GLvoid (*_GLUfuncptr)(void);
#endif
#endif


// definitions from lib_main
typedef int BOOL;
#ifndef bool
typedef int bool;
#endif /* bool */

#ifndef FALSE
#define FALSE 0
#endif

#ifndef TRUE
static int TRUE = (!0);
#endif

/* 68000 Register defines */
enum {
  REG_D0,    /* D0.. */
  REG_D1,
  REG_D2,
  REG_D3,
  REG_D4,
  REG_D5,
  REG_D6,
  REG_D7,    /* ..D7 */
  REG_A0,    /* A0.. */
  REG_A1,
  REG_A2,
  REG_A3,
  REG_A4,
  REG_A5,
  REG_A6,
  REG_A7,    /* ..A7(also SP) */
};

// addiotional screen.c exports
struct ZNode {
	unsigned int z;
	struct ZNode *less, *more;
	void *data;
};

extern GLUquadricObj *qobj;
extern GLUtesselator *tobj;
extern unsigned int screen_tex;
extern struct ZNode *znode_start;
extern struct ZNode *znode_cur;

extern void CALLBACK beginCallback(GLenum which);
extern void CALLBACK errorCallback(GLenum errorCode);
extern void CALLBACK endCallback(void);
extern void CALLBACK vertexCallback(GLvoid *vertex, GLvoid *poly_data);
extern void CALLBACK combineCallback(GLdouble coords[3],
                     GLdouble *vertex_data[4],
                     GLfloat weight[4], GLdouble **dataOut );

extern void end_node();
extern void draw_3dview (struct ZNode *node);
extern void draw_control_panel();
extern void set_gl_clear_col(int rgb);
extern void set_main_viewport();
#include "screen.h"

#include "input.h"
