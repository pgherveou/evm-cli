# Recording Review Guide

## How to Review This Recording

### 1. View ASCII Recording
The `.ascii` file contains a text-based recording of the terminal session. You can:

**Option A: Direct View**
```bash
cat spec/$SPEC_NAME/recordings/$FEATURE_NAME.ascii
```

**Option B: VHS Playback**
```bash
vhs play spec/$SPEC_NAME/recordings/$FEATURE_NAME.ascii
```

### 2. Review Tape Script
Check the `.tape` file to understand the recorded interactions:
```bash
cat spec/$SPEC_NAME/recordings/$FEATURE_NAME.tape
```

### 3. Compare to Specification
1. Open `spec/$SPEC_NAME/spec.md`
2. Verify the recording demonstrates all key features from the spec
3. Check that keyboard shortcuts work as documented
4. Verify visual styling matches specification mockups

## Review Checklist

### Visual Elements
- [ ] UI layout matches spec mockups
- [ ] Colors and highlighting correct (cyan selection, status colors)
- [ ] Spacing and alignment follow design system
- [ ] All interactive elements visible and responsive

### Keyboard Navigation
- [ ] Arrow keys work as documented
- [ ] Vim keys (hjkl) work as documented
- [ ] Tab switching works between areas
- [ ] Enter executes actions
- [ ] Escape cancels/closes modals

### Feature-Specific
- [ ] Feature behavior matches spec description
- [ ] Error messages display correctly
- [ ] Status feedback is clear
- [ ] Loading states show properly
- [ ] Transitions are smooth

### Quality
- [ ] Recording is clear and readable
- [ ] No glitches or rendering issues
- [ ] Timing is realistic (not too fast/slow)
- [ ] All steps are visible and understandable

## Issues Found

If you find issues with the recording or implementation:

1. **Recording Issues**: Re-run the recording script
   ```bash
   ./scripts/record-spec-tape.sh $SPEC_NAME $FEATURE_NAME
   ```

2. **Implementation Issues**: File a bug report with:
   - Spec name and feature
   - Description of the issue
   - Steps to reproduce
   - Expected vs actual behavior

## Recording Regeneration

To regenerate this recording:
```bash
./scripts/record-spec-tape.sh $SPEC_NAME $FEATURE_NAME
```

## Related Files
- Spec: `spec/$SPEC_NAME/spec.md`
- Tape: `spec/$SPEC_NAME/recordings/$FEATURE_NAME.tape`
- ASCII: `spec/$SPEC_NAME/recordings/$FEATURE_NAME.ascii`
- Video: `spec/$SPEC_NAME/recordings/$FEATURE_NAME.mp4`

