/*******************************************************************************************
*
*   raylib [core] example - Basic 3d example
*
*   Welcome to raylib!
*
*   To compile example, just press F5.
*   Note that compiled executable is placed in the same folder as .c file
*
*   You can find all basic examples on C:\raylib\raylib\examples folder or
*   raylib official webpage: www.raylib.com
*
*   Enjoy using raylib. :)
*
*   This example has been created using raylib 1.0 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2013-2020 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

#include <stdio.h>
//#include "raylib.h"
#include "gameboy.h"

extern struct registers_t registers;

int main()
{
    // Initialization
    //--------------------------------------------------------------------------------------
    const int screenWidth = 400;//160;
    const int screenHeight = 400;//144;

    //InitWindow(screenWidth, screenHeight, "raylib");

    //Camera camera = { 0 };
    //camera.position = (Vector3){ 10.0f, 10.0f, 8.0f };
    //camera.target = (Vector3){ 0.0f, 0.0f, 0.0f };
    //camera.up = (Vector3){ 0.0f, 1.0f, 0.0f };
    //camera.fovy = 60.0f;
    //camera.projection = CAMERA_PERSPECTIVE;
    
    //SetCameraMode(camera, CAMERA_ORBITAL);

    //Vector3 cubePosition = { 0 };

    //SetTargetFPS(60);               // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    //loadRom("testroms/blarrg/cpu_instrs/individual/01-special.gb");
    //loadRom("testroms/blarrg/cpu_instrs/individual/02-interrupts.gb");
    //loadRom("testroms/blarrg/cpu_instrs/individual/03-op sp,hl.gb");
    //loadRom("testroms/blarrg/cpu_instrs/individual/04-op r,imm.gb");
    //loadRom("testroms/blarrg/cpu_instrs/individual/05-op rp.gb");
    //loadRom("testroms/blarrg/cpu_instrs/individual/06-ld r,r.gb");
    loadRom("testroms/blarrg/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb");
    //loadRom("testroms/blarrg/cpu_instrs/individual/08-misc instrs.gb");
    //loadRom("testroms/blarrg/cpu_instrs/individual/09-op r,r.gb");
    //loadRom("testroms/blarrg/cpu_instrs/individual/10-bit ops.gb");
    //loadRom("testroms/blarrg/cpu_instrs/individual/11-op a,(hl).gb");

    //loadRom("testroms/games/Tetris\ (World)\ (Rev A).gb");
    //loadRom("testroms/games/Dr.\ Mario(JU)\ (V1.1).gb");

    reset(); // Initialize gameboy state

    printf("\n\n");
    // Main game loop
    while (1)//!WindowShouldClose())    // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        //UpdateCamera(&camera);
        //----------------------------------------------------------------------------------

        step();

        // Draw
        //----------------------------------------------------------------------------------
        /*BeginDrawing();

            ClearBackground(RAYWHITE);

            char buff[256];

            sprintf(buff, "AF: %04X", registers.af);
            DrawText(buff, 5, 5, 10, BLACK);
            sprintf(buff, "BC: %04X", registers.bc);
            DrawText(buff, 5, 15, 10, BLACK);
            sprintf(buff, "DE: %04X", registers.de);
            DrawText(buff, 5, 25, 10, BLACK);
            sprintf(buff, "HL: %04X", registers.hl);
            DrawText(buff, 5, 35, 10, BLACK);
            sprintf(buff, "SP: %04X", registers.sp);
            DrawText(buff, 5, 45, 10, BLACK);
            sprintf(buff, "PC: %04X", registers.pc);
            DrawText(buff, 5, 55, 10, BLACK);

            DrawFPS(10, 350);


        EndDrawing();*/
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    //CloseWindow();        // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}