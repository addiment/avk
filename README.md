# AVK

Addie's Virtual Console!

Currently a very messy preview. Proper documentation will come once the API is hammered out a little more.

Task List:
- Reorganize the GK and SDL subsystems (mutability rules exist for a reason!)
- Rewrite the ABI to avoid having resources loaded programmatically (remove AVK_INIT, AVK_DROP, and load resources straight from the ROM)
- Improve controller support
- Create unit and integration tests
- Add background rendering
  - XY pan,
  - function-based distortion? to replicate interrupt timing on early consoles (like Earthbound backgrounds)
- Sequenced audio