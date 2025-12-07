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

# Deleted Files - NTFS

When a file is deleted:
    - $MFT marks the FILE entry as available for re-use in its tracking attribute
    - $DATA attribute of FILE entry is read, $BITMAP is updated to show this memory.

Nothing is wiped. Until the FILE entry is overwritten, the the data is still stored in the same location
pointed at by $DATA
