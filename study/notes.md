# Expert Witness Format
                                                Data blocks form the MD5/SHA1 Checksum, so that the hash will matcha
                                                that of a raw data dump/the original evidence
                                                /-------------------------------------------------------------------------------------\
                                                |                            |                            |                           |
                                                |                            |                            |                           v
|------------------------|--------------|--------------|-------------|--------------|-------------|--------------|--------------|------------|
|       Header           |              |              |             |              |             |              |              |            |
|   comprised by:        |              |              |             |              |             |              |              |            |
|    - Case Name         |              |              |             |              |             |              |              |            |
|    - Investigator      |  Header CRC  |  Data Block  |  Block CRC  |  Data Block  |  Block CRC  |  Data Block  |  Block CRC   |  MD5/SHA1  |
|    - Evidence Name     |              |              |             |              |             |              |              |            |
|    - Evidence #        |              |              |             |              |             |              |              |            |
|    - Evidence Notes    |              |              |             |              |             |              |              |            |
|------------------------|--------------|--------------|-------------|--------------|-------------|--------------|--------------|------------|
            |                   |               |             |              |             |               |             |
            |                   |               |             |              |             |               |             |
            \-------------------/               \-------------/              \-------------/               \-------------/
            The header and every block are associated with a Cyclic Redundancy Check Checksum.
            A cyclic Redundancy Check Checksum is an algorithm to detect errors in data transmission/storage.
            By treating some associated data as a polynomial, dividing it by a standard generator polynomial
            and using the remainder (the CRC Checksum) to verify the data integrity at the receiving end.

Collisions are statistically unlikely because there would need to be a collision in both the MD5/SHA1 Checksum and the
CRC Checksum at the same time

# Acquisition Documentation

During a device acquisition, you must record:
    - Device Model
    - Device Serial Number
    - If Hard Disk, Disk Model & Serial Number
    - Acquisition Hash(es)

You must log the start and stop times of the acquisition.
A secondary verifier must sign off on the acquisition.

# Drive

Traditional drives are arranged into a number of circular platters,
each containing a number of concentric tracks.

Logical Block Addressing (LBA) is a linear addressing scheme where blocks are
located by an integer index, the first being LBA0, the second LBA2, etc. This differs
in comparison to CHS Addressing (Cylinder, Head, Sector) which identified the physical
location of the data by using the CHS physical components as 3-point co-ordinate system.

Due to the contiguous arrangement memory in LBA, File Carving and recovery tools work exceptionally well.

# Drive Logical Structure

|---------------------------------|
|                                 |
|   |------------------------|    |
|   |       LBA 0:           |    |     The MBR contains the volume locations and sizes
|   |  Master Boot Record    |    |
|   |------------------------|    |
|   |  Logical Volume 1 (C:) |    |     Volumes can can be empty or hold a filesystem
|   |  Logical Container     |    |
|   |  for a filesystem      |    |
|   |------------------------|    |
|   |  Logical Volume 2 (D:) |    |     Filesystems sit within a volume and allow files
|   |           D:           |    |     to be organised and saved
|   |  Logical Container     |    |
|   |  for a filesystem      |    |
|   |------------------------|    |
|          Unparitioned           |     If you don't assign all sectors to a partition, they
|             Space               |     will sit within the unpartitioned space on the drive
|---------------------------------|

# Acquisition - Physical vs. Logical vs. LEF

A physical acquisition is a bit-for-bit copy of a device (/dev/nvme0n1). Capturing all areas of the disk,
including unpartitioned space, volume slack and other slack space areas. Supported by Expert Witness Format.

A logical acquisition is a copy of a selected partition (/dev/nvme0n1p1). Most common during server acquisition.
Supported by Expert Witness Format.

A Logical Evidence File (LEF) is a forensic image of a collection of files or folders that is used in scenarios
where capturing the physical disks is not possible or practical. Not supported by EWF, requires L01/AD1

# Volume Slack

|---------------------------------|
|                                 |
|   |------------------------|    |     Volume slack is the unused space between the end of the filesystem and
|   |       LBA 0:           |    |     the end of the partition where the filesystem resides. In hard disk terms,
|   |  Master Boot Record    |    |     sectors at the end of the volume or partition that cannot be allocated to a
|   |------------------------|    |     cluster. This happens when the partition size is not a multiple of the cluster
|   |  Logical Volume 1 (C:) |    |     size.
|   |                        |    |
|   | |--------------------| |    |     Lets assume that we create a partition with 100 sectors. You create an NTFS
|   | |                    | |    |     filesystem in that partition with 15 sectors per cluster. 100 % 15 = 10.
|   | |  NTFS Filesystem   | |    |     These 10 sectors aren't assigned to a cluster within the filesystem and are unused.
|   | |                    | |    |
|   | |--------------------| |    |     With a sector size of 512 bytes, these 10 sectors equate to 512 * 10 = 5KB
|   |                        |    |     volume slack.
|   |     Volume Slack       |    |
|   |                        |    |
|   |------------------------|    |
|          Unparitioned           |
|             Space               |
|---------------------------------|

# NTFS Fileystem

In NTFS, everything is a file, by virtue of the $MFT (Master File Table).
The $MFT is a hidden system file that acts as a database, storing metadata
(names, sizes, timestamps, permissions, locations) on every file and folder
on a drive, including itself, essentially defining what constitutes a file and its properties

# FILE Records - NTFS

|------------------------|
|       FILE0...         |      MFT Header
|------------------------|
|  $STANDARD_INFORMATION |      contains created, modified,
|                        |      changed and accessed times
|------------------------|
|  $FILENAME             |      The filename
|                        |
|------------------------|
|  $DATA                 |      The contents of the file. Starts 'resident' inside the entry itself.
|                        |      When it grows too large, becomes 'non-resident' and is moved to disk.
|------------------------|

# File Carving

File carving scans all clusters that are unallocated for traces of files that once existed in that space.

# Deleted Files - NTFS

When a file is deleted:
    - $MFT marks the FILE entry as available for re-use in its tracking attribute
    - $DATA attribute of FILE entry is read, $BITMAP is updated to show this memory.

Nothing is wiped. Until the FILE entry is overwritten, the the data is still stored in the same location
pointed at by $DATA

# Fle Slack

File Slack Occurs because data can only be allocated to files at the Cluster level. If a file does not fill the entire
cluster, the remaining “slack” may contain residual or hidden data. This is also known as Drive Slack.

# Registry Hives

• SAM – Security Accounts Manager, contains user and group membership info.
• SYSTEM – contains information about the Windows system setup, the list of currently
mounted devices containing a filesystem, configurations for system hardware drivers and
services running on the local system.
• SOFTWARE - contains software and Windows settings. It is mostly modified by application
and system installers
• SECURITY - The kernel will access it to read and enforce the security policy applicable to the
current user and all applications or operations executed by this user.
The is also a per user hive
• NTUSER.dat

# USB Drives

Under the SYSTEM hive, the USBSTOR key contains a subkey for each USB device plugged into the system
    - USB ClassGUID
    - Manufacturer Name, Friendly Device Name
    - Serial Number

Plug and Play Log - C:\Windows\setupapi.log

# Linkfiles

Linkfiles are small “shortcut” files used by windows for features such as “Recents”. They frequently get created
when a file is opened from Windows Explorer.
They typically have an “lnk” file extention, can can contain information such as:
- The orginal path and filename of file, including the drive letter, or network path.
- The timestamps of the target file (in addition to its own timestamps)
- For external drives, the volume serial number of the drive.

# Prefetch

The Prefetch folder was introduced in Windows XP, and was designed to speed up the application
startup process.
The Prefetch folder is located at C:\Windows\Prefetch
Each prefetch file contains:
- The name of the executable
- Unicode list of DLLs used by that executable
- A count of how many times the executable has
been run
- A timestamp indicating the last time the program
was run.

# Memory Analytics

(1) Investigate running processes
(2) Identify suspicious file handles, DLLs
(3) Observe network traffic/ports
(4) Identify code injection
(5) Investigate possible root kits
(6) Export for further analysis

vol -f <file> windows.pslist or windows.pstree

# 
