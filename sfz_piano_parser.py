#!/usr/bin/env python3
"""
Piano-optimized SFZ Parser
Creates JSON files organized by MIDI keys (0-87) for easy virtual piano querying
"""

import json
import re
import os
from pathlib import Path
from typing import Dict, List, Any


class PianoSFZParser:
    def __init__(self):
        self.variables = {}
        self.vel_variables = {}  # Store all VEL variables separately
        self.vel_mappings = {}  # Store velocity range to VEL variable mapping
        self.base_path = None
        self.parsed_files = set()
        
    def parse_instrument(self, instrument_name: str, sfz_path: str) -> Dict[str, Any]:
        """Parse SFZ file and create piano-optimized JSON structure"""
        self.base_path = os.path.dirname(sfz_path)
        self.parsed_files = set([os.path.abspath(sfz_path)])
        
        with open(sfz_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Extract description before removing comments
        description = self._extract_description_from_original(content)
        
        # Reset state
        self.variables = {}
        
        # Process the file with proper variable substitution
        processed_content = self._process_file(content)
        
        # Extract the three sections
        metadata = self._extract_metadata(processed_content, instrument_name)
        key_mappings = self._extract_key_mappings(processed_content)
        
        return {
            "description": description,
            "metadata": metadata,
            "keys": key_mappings
        }
    
    def _process_file(self, content: str) -> str:
        """Process SFZ file: remove comments, parse defines, process includes"""
        # Remove comments
        content = re.sub(r'//.*', '', content)
        
        # Parse defines from this content
        self._parse_defines(content)
        
        # Build velocity range to VEL mapping from notes.txt structure
        self._build_vel_mapping_from_notes(content)
        
        # Substitute variables in the content (including include paths)
        content = self._substitute_variables(content)
        
        # Process includes recursively
        content = self._process_includes(content)
        
        # Parse any new defines from included content
        self._parse_defines(content)
        
        # Final variable substitution for any remaining variables
        content = self._substitute_variables(content)
        
        return content
    
    def _build_vel_mapping_from_notes(self, content: str):
        """Build velocity range to VEL mapping from notes.txt structure"""
        # Parse the notes.txt structure to extract velocity mappings
        notes_pattern = r'<group>\s+#include\s+"Data/vel_(\d+)\.txt"\s+lovel=(\d+)\s+hivel=(\d+)\s+#include\s+"Data/region\.txt"'
        
        for match in re.finditer(notes_pattern, content):
            vel_file = f"vel_{match.group(1)}.txt"
            lovel = int(match.group(2))
            hivel = int(match.group(3))
            vel_range = f"{lovel}-{hivel}"
            
            # Map this velocity range to the VEL file
            self.vel_mappings[vel_range] = vel_file
    
    def _parse_defines(self, content: str):
        """Parse #define statements"""
        define_pattern = r'#define\s+\$(\w+)\s+(.+)'
        for match in re.finditer(define_pattern, content):
            var_name = match.group(1)
            var_value = match.group(2).strip()
            self.variables[var_name] = var_value
            
            # Store VEL variables separately
            if var_name.startswith('VEL'):
                self.vel_variables[var_name] = var_value
    
    def _substitute_variables(self, content: str) -> str:
        """Substitute variables in content"""
        # Use word boundaries to avoid corrupting #define statements
        for var_name, var_value in self.variables.items():
            # Replace only when $VAR appears as a whole word, not in #define statements
            pattern = r'\$\{' + var_name + r'\}'  # ${VAR} format
            content = re.sub(pattern, var_value, content)
            
            pattern = r'\$' + var_name + r'\b'  # $VAR format (word boundary)
            content = re.sub(pattern, var_value, content)
        return content
    
    def _process_includes(self, content: str) -> str:
        """Process #include statements recursively"""
        include_pattern = r'#include\s+"([^"]+)"'
        
        def replace_include(match):
            include_path = match.group(1)
            full_path = os.path.join(self.base_path, include_path)
            
            if os.path.exists(full_path):
                abs_path = os.path.abspath(full_path)
                if abs_path in self.parsed_files:
                    return f"// Circular include avoided: {include_path}"
                
                self.parsed_files.add(abs_path)
                
                with open(full_path, 'r', encoding='utf-8') as f:
                    included_content = f.read()
                
                # Remove comments from included content
                included_content = re.sub(r'//.*', '', included_content)
                
                # Parse any new defines from included content
                self._parse_defines(included_content)
                
                # Substitute variables (including any new ones from includes)
                included_content = self._substitute_variables(included_content)
                
                # Process nested includes
                included_content = self._process_includes(included_content)
                
                return included_content
            else:
                print(f"Warning: Include file not found: {full_path}")
                return f"// Missing include: {include_path}"
        
        return re.sub(include_pattern, replace_include, content)
    
    def _extract_description_from_original(self, content: str) -> str:
        """Extract description from original SFZ file comments"""
        lines = content.split('\n')
        description_parts = []
        
        for line in lines:
            line = line.strip()
            if line.startswith('//'):
                comment = line[2:].strip()
                if (comment and 
                    not all(c == '-' for c in comment) and
                    not comment.startswith('midi cc') and
                    not comment.startswith('file-folder') and
                    not comment.startswith('////////////////////////////////////////////////////') and
                    not comment.startswith('*************************************************') and
                    not 'https://' in comment):
                    description_parts.append(comment)
        
        if description_parts:
            description = ' '.join(description_parts)
            return description[:200] + ('...' if len(description) > 200 else '')
        else:
            return "Piano instrument SFZ file"
    
    def _extract_metadata(self, content: str, instrument_name: str) -> Dict[str, Any]:
        """Extract metadata specific to each instrument"""
        metadata = {
            "instrumentName": instrument_name,
            "fileExtension": "flac",
            "keyCount": 88  # Standard piano has 88 keys
        }
        
        # Extract control information
        control_section = re.search(r'<control>(.*?)(?=<|\Z)', content, re.DOTALL)
        if control_section:
            control_content = control_section.group(1)
            
            # Extract MIDI CC mappings
            cc_mappings = {}
            cc_pattern = r'label_cc(\d+)=([^\n]+)'
            for match in re.finditer(cc_pattern, control_content):
                cc_num = match.group(1)
                cc_label = match.group(2).strip()
                cc_mappings[cc_num] = cc_label
            
            if cc_mappings:
                metadata["midiCc"] = cc_mappings
            
            # Extract default path
            default_path_match = re.search(r'default_path=([^\n]+)', control_content)
            if default_path_match:
                metadata["samplePath"] = default_path_match.group(1).strip()
        
        # Add instrument-specific metadata
        if instrument_name == "splendid":
            metadata.update({
                "type": "Splendid Grand Piano",
                "publicDomain": True,
                "halfPedaling": True,
                "stringResonance": True
            })
        elif instrument_name == "salamander":
            metadata.update({
                "type": "Salamander Grand Piano V3",
                "yamahaC5": True,
                "retunedVersion": True,
                "keyswitches": ["Natural", "Retuned"]
            })
        
        return metadata
    
    def _extract_key_mappings(self, content: str) -> Dict[str, Any]:
        """Extract sample mappings organized by piano keys (21-108) like SFZ file"""
        # Initialize all 88 keys
        keys = {}
        for midi_note in range(21, 109):  # A0 (21) to C8 (108)
            keys[str(midi_note)] = {
                "midiNote": midi_note,
                "noteName": self._midi_to_note_name(midi_note),
                "samples": []
            }
        
        # Find all regions with sample information
        region_pattern = r'<region>\s+([^\n]+)'
        
        for match in re.finditer(region_pattern, content):
            region_line = match.group(1)
            
            # Parse opcodes from region
            opcodes = {}
            opcode_pattern = r'(\w+)=([^=]+?)(?=\s+\w+=|$)'
            
            for opcode_match in re.finditer(opcode_pattern, region_line):
                key = opcode_match.group(1)
                value = opcode_match.group(2).strip()
                opcodes[key] = value
            
            # Extract sample information
            if 'sample' in opcodes:
                lokey = int(opcodes.get('lokey', '0'))
                hikey = int(opcodes.get('hikey', '127'))
                lovel = int(opcodes.get('lovel', '0'))
                hivel = int(opcodes.get('hivel', '127'))
                pitch_center = int(opcodes.get('pitch_keycenter', lokey))
                
                # Only process piano keys (21-108)
                if lokey <= 108 and hikey >= 21:
                    # Get velocity layer name from variables
                    vel_layer_name = None
                    if 'VEL' in self.vel_variables:
                        # For salamander: VEL01, VEL02, etc.
                        # Direct mapping based on velocity ranges
                        if lovel == 1 and hivel == 26:
                            vel_layer_name = "VEL01"
                        elif lovel == 27 and hivel == 34:
                            vel_layer_name = "VEL02"
                        elif lovel == 35 and hivel == 36:
                            vel_layer_name = "VEL03"
                        elif lovel == 37 and hivel == 43:
                            vel_layer_name = "VEL04"
                        elif lovel == 44 and hivel == 46:
                            vel_layer_name = "VEL05"
                        elif lovel == 47 and hivel == 50:
                            vel_layer_name = "VEL06"
                        elif lovel == 51 and hivel == 56:
                            vel_layer_name = "VEL07"
                        elif lovel == 57 and hivel == 64:
                            vel_layer_name = "VEL08"
                        elif lovel == 65 and hivel == 72:
                            vel_layer_name = "VEL09"
                        elif lovel == 73 and hivel == 80:
                            vel_layer_name = "VEL10"
                        elif lovel == 81 and hivel == 88:
                            vel_layer_name = "VEL11"
                        elif lovel == 89 and hivel == 96:
                            vel_layer_name = "VEL12"
                        elif lovel == 97 and hivel == 104:
                            vel_layer_name = "VEL13"
                        elif lovel == 105 and hivel == 112:
                            vel_layer_name = "VEL14"
                        elif lovel == 113 and hivel == 120:
                            vel_layer_name = "VEL15"
                        elif lovel == 121 and hivel == 127:
                            vel_layer_name = "VEL16"
                        else:
                            vel_layer_name = f"{lovel}-{hivel}"
                    elif 'DYN' in self.variables:
                        # For splendid: PP, MF, FF
                        vel_layer_name = self.variables['DYN']
                    else:
                        # Fallback to numeric range
                        vel_layer_name = f"{lovel}-{hivel}"
                    
                    # Create sample info
                    sample_info = {
                        "file": opcodes['sample'],
                        "keyRange": {
                            "low": lokey,
                            "high": hikey
                        },
                        "velocityRange": vel_layer_name
                    }
                    
                    # Add optional parameters
                    if 'volume' in opcodes:
                        sample_info["volume"] = opcodes['volume']
                    if 'tune' in opcodes:
                        sample_info["tune"] = opcodes['tune']
                    if 'region_label' in opcodes:
                        sample_info["label"] = opcodes['region_label']
                    
                    # Add this sample to all keys in the range
                    for midi_note in range(max(lokey, 21), min(hikey, 108) + 1):
                        keys[str(midi_note)]["samples"].append(sample_info)
        
        # Sort samples by velocity for each key
        for midi_note in keys:
            def get_sort_key(sample):
                vel_range = sample["velocityRange"]
                if isinstance(vel_range, str):
                    # Extract numeric part from strings like "VEL01" or "PP"
                    import re
                    numbers = re.findall(r'\d+', vel_range)
                    return int(numbers[0]) if numbers else 0
                else:
                    return vel_range.get("low", 0)
            
            keys[midi_note]["samples"].sort(key=get_sort_key)
        
        return keys
    
    def _midi_to_note_name(self, midi_note: int) -> str:
        """Convert MIDI note number to note name"""
        note_names = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B']
        octave = (midi_note // 12) - 1
        note_name = note_names[midi_note % 12]
        return f"{note_name}{octave}"


def main():
    """Main function to parse all SFZ files"""
    base_dir = "/home/emiya2467/Projects/raku/src-tauri/data"
    map_dir = "/home/emiya2467/Projects/raku/src-tauri/data/map"
    
    instruments = [
        {
            'name': 'splendid',
            'sfz_path': os.path.join(base_dir, 'splendid', 'Splendid Grand Piano.sfz')
        },
        {
            'name': 'salamander',
            'sfz_path': os.path.join(base_dir, 'salamander', 'Salamander Grand Piano V3.sfz')
        }
    ]
    
    # Ensure output directory exists
    os.makedirs(map_dir, exist_ok=True)
    
    parser = PianoSFZParser()
    
    for instrument in instruments:
        if os.path.exists(instrument['sfz_path']):
            print(f"Parsing {instrument['name']}...")
            
            try:
                result = parser.parse_instrument(instrument['name'], instrument['sfz_path'])
                
                # Save to JSON
                output_path = os.path.join(map_dir, f"{instrument['name']}.json")
                with open(output_path, 'w', encoding='utf-8') as f:
                    json.dump(result, f, indent=2, ensure_ascii=False)
                
                # Count keys with samples
                keys_with_samples = len([k for k in result["keys"].values() if k["samples"]])
                total_samples = sum(len(k["samples"]) for k in result["keys"].values())
                
                print(f"  - Description: {result['description'][:50]}...")
                print(f"  - Keys with samples: {keys_with_samples}/88")
                print(f"  - Total samples: {total_samples}")
                print(f"  - Saved to: {output_path}")
                
            except Exception as e:
                print(f"  Error: {e}")
                import traceback
                traceback.print_exc()
        else:
            print(f"SFZ file not found: {instrument['sfz_path']}")
    
    print(f"\nParsing complete! Check {map_dir} for JSON files.")


if __name__ == "__main__":
    main()
